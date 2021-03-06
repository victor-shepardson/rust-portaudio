// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::libc::{c_char, c_double, c_void};

use types::*;
//use mac_core::*;

extern "C" {

    /// PortAudio portable API

    pub fn Pa_GetVersion() -> i32;
    pub fn Pa_GetVersionText() -> *c_char;
    pub fn Pa_GetErrorText(errorCode : PaError) -> *c_char;
    pub fn Pa_Initialize() -> PaError;
    pub fn Pa_Terminate() -> PaError;
    pub fn Pa_GetHostApiCount() -> PaHostApiIndex;
    pub fn Pa_GetDefaultHostApi() -> PaHostApiIndex;
    pub fn Pa_GetHostApiInfo(hostApi : PaHostApiIndex) -> *C_PaHostApiInfo;
    pub fn Pa_HostApiTypeIdToHostApiIndex(type_id : PaHostApiTypeId) -> PaHostApiIndex;
    pub fn Pa_HostApiDeviceIndexToDeviceIndex(hostApi : PaHostApiIndex, hostApiDeviceIndex : i32) -> PaDeviceIndex;
    pub fn Pa_GetLastHostErrorInfo() -> *C_PaHostErrorInfo;
    pub fn Pa_GetDeviceCount() -> PaDeviceIndex;
    pub fn Pa_GetDefaultInputDevice() -> PaDeviceIndex;
    pub fn Pa_GetDefaultOutputDevice() -> PaDeviceIndex;
    pub fn Pa_GetDeviceInfo(device : PaDeviceIndex) -> *C_PaDeviceInfo;
    pub fn Pa_IsFormatSupported(input_parameters : *C_PaStreamParameters, outputParameters : *C_PaStreamParameters, sampleRate : c_double) -> PaError;
    pub fn Pa_GetSampleSize(format : PaSampleFormat) -> PaError;
    pub fn Pa_Sleep(msec : i32) -> ();
    pub fn Pa_OpenStream(stream : **C_PaStream, 
                         inputParameters : *C_PaStreamParameters, 
                         outputParameters : *C_PaStreamParameters, 
                         sampleRate : c_double, 
                         framesPerBuffer : u32, 
                         streamFlags : PaStreamFlags, 
                         streamCallback : Option<extern "C" fn(*c_void, *c_void, u32, *PaStreamCallbackTimeInfo, PaStreamCallbackFlags, *c_void) -> PaStreamCallbackResult>, 
                         userData : *c_void) 
                         -> PaError;
    pub fn Pa_OpenDefaultStream(stream : **C_PaStream, 
                                numInputChannels : i32, 
                                numOutputChannels : i32, 
                                sampleFormat : PaSampleFormat, 
                                sampleRate : c_double, 
                                framesPerBuffer : u32, 
                                streamCallback : Option<extern "C" fn(*c_void, *c_void, u32, *PaStreamCallbackTimeInfo, PaStreamCallbackFlags, *c_void) -> PaStreamCallbackResult>, 
                                userData : *c_void)
                                -> PaError;
    pub fn Pa_CloseStream(stream : *C_PaStream) -> PaError;
    //pub fn Pa_SetStreamFinishedCallback (stream : *PaStream, PaStreamFinishedCallback *streamFinishedCallback) -> PaError;
    pub fn Pa_StartStream(stream : *C_PaStream) -> PaError;
    pub fn Pa_StopStream(stream : *C_PaStream) -> PaError;
    pub fn Pa_AbortStream(stream : *C_PaStream) -> PaError;
    pub fn Pa_IsStreamStopped(stream : *C_PaStream) -> PaError;
    pub fn Pa_IsStreamActive(stream : *C_PaStream) -> i32;
    pub fn Pa_GetStreamInfo(stream : *C_PaStream) -> *PaStreamInfo;
    pub fn Pa_GetStreamTime(stream : *C_PaStream) -> PaTime;
    pub fn Pa_GetStreamCpuLoad(stream : *C_PaStream) -> c_double;
    pub fn Pa_ReadStream(stream : *C_PaStream, buffer : *c_void, frames : u32) -> PaError;
    pub fn Pa_WriteStream(stream : *C_PaStream, buffer : *c_void, frames : u32) -> PaError;
    pub fn Pa_GetStreamReadAvailable(stream : *C_PaStream) -> i64;
    pub fn Pa_GetStreamWriteAvailable(stream : *C_PaStream) -> i64;

    /*
    * PortAudio Specific ASIO
    */
    pub fn PaAsio_GetAvailableBufferSizes(device : PaDeviceIndex, minBufferSizeFrames : *i32, maxBufferSizeFrames : *i32, preferredBufferSizeFrames : *i32, granularity : *i32) -> PaError;
    pub fn PaAsio_GetInputChannelName(device : PaDeviceIndex, channelIndex : i32, channelName : **c_char) -> PaError;
    pub fn PaAsio_GetOutputChannelName(device : PaDeviceIndex, channelIndex : i32, channelName : **c_char) -> PaError;
    pub fn PaAsio_SetStreamSampleRate(stream : *C_PaStream, sampleRate : c_double) -> PaError;


    /*
    * PortAudio Specific MAC_CORE
    */
    pub fn PaMacCore_GetStreamInputDevice(s : *C_PaStream) -> PaDeviceIndex;
    pub fn PaMacCore_GetStreamOutputDevice(s : *C_PaStream) -> PaDeviceIndex;
    // pub fn PaMacCore_GetChannelName (int device, int channelIndex, bool intput) -> *c_char
    pub fn PaMacCore_GetBufferSizeRange(device : PaDeviceIndex, minBufferSizeFrames : *u32, maxBufferSizeFrames : *u32) -> PaError;
    //pub fn PaMacCore_SetupStreamInfo(PaMacCoreStreamInfo *data, unsigned long flags) -> ();
    //pub fn PaMacCore_SetupChannelMap(PaMacCoreStreamInfo *data, const SInt32 *const channelMap, unsigned long channelMapSize) -> ();
}
