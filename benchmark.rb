# frozen_string_literal: true

require 'ffi'
require 'benchmark'
require 'mini_magick'

# wraps a rust image processing library
module RustImageProcessing
  extend FFI::Library
  ffi_lib 'target/release/libimage_processing.dylib' # MacOS -> .dylib, Linux -> .so, Windows -> .dll

  attach_function :resize_image, %i[string string uint32], :void
end

def resize_image_with_mini_magick(input, output, new_width)
  image = MiniMagick::Image.open(input)
  image.resize "#{new_width}x#{new_width}"
  image.write output
end

n = 50
Benchmark.bm do |x|
  x.report('MiniMagick: ') { n.times { resize_image_with_mini_magick('input.jpg', 'output.jpg', 50); } }
  x.report('Rust: ') { n.times { RustImageProcessing.resize_image('input.jpg', 'output.jpg', 50) } }
end
