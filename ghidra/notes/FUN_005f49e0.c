
undefined * __cdecl FUN_005f49e0(uint param_1)

{
  HRSRC hResInfo;
  HGLOBAL hResData;
  char *_Source;
  
  hResInfo = FindResourceA(DAT_006bdf18,(LPCSTR)(param_1 & 0xffff),(LPCSTR)0xa);
  if (hResInfo != (HRSRC)0x0) {
    hResData = LoadResource(DAT_006bdf18,hResInfo);
    _Source = (char *)LockResource(hResData);
    _strncpy(&DAT_006bde18,_Source,0xff);
    FreeResource(hResData);
    DAT_006bdf17 = 0;
    return &DAT_006bde18;
  }
  DAT_006ac240 = 4;
  DAT_006bde18 = 0;
  return &DAT_006bde18;
}

