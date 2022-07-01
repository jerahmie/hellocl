#include <iostream>
#include <vector>
#include <string>

#define __CL_ENABLE_EXCEPTIONS
#include <CL/cl.hpp>

// compute c = a + b
static const char source[] =
  "#if define(dcl_khr_fp64)\n"
  "#  pragma OPENCL EXTENSION cl_khr_fp64: enable\n"
  "#elif defined(cl_amp_fp64)\n"
  "#  pragma OPENCL EXTENSION cl_amp_fp64: enable\n"
  "#else\n"
  "#  error double precision is not supported\n"
  "#endif\n"
  "kernel void add(\n"
  "       ulong n,\n"
  "       global const double *a,\n"
  "       global const double *b,\n"
  "       global double *c\n"
  "       )\n"
  "{\n"
  "    size_t i = get_global_id(0);\n"
  "    if (i < n) {\n"
  "       c[i] = a[i] + b[i];\n"
  "    }\n"
  "}\n";
  

int main(void)
{
  const size_t N = 1 << 20;
  
  try {
    // Get list of OpenCL platforms.
    std::vector<cl::Platform> platform;
    cl::Platform::get(&platform);

    if (platform.empty()) {
      std::cerr << "OpenCL platforms not found." << '\n';
      return 1;
    }
  } catch (const cl::Error &err) {
    std::cerr
      << "OpenCL error: "
      << err.what() << "(" << err.err() << ")"
      << std::endl;
    return 1;
  }
}
