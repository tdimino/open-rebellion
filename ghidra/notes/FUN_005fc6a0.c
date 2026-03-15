
undefined4 __thiscall FUN_005fc6a0(void *this,LPCSTR param_1)

{
  HRSRC hResInfo;
  HGLOBAL hResData;
  LPVOID pvVar1;
  
  if ((*(HMODULE *)((int)this + 0x18) != (HMODULE)0x0) && (param_1 != (LPCSTR)0x0)) {
    if (*(int *)((int)this + 0x14) != 0) {
      *(undefined4 *)((int)this + 0x14) = 0;
    }
    hResInfo = FindResourceA(*(HMODULE *)((int)this + 0x18),param_1,&DAT_00000002);
    hResData = LoadResource(*(HMODULE *)((int)this + 0x18),hResInfo);
    *(HGLOBAL *)((int)this + 0x14) = hResData;
    if (hResData != (HGLOBAL)0x0) {
      pvVar1 = LockResource(hResData);
      *(LPVOID *)this = pvVar1;
    }
                    /* WARNING: Load size is inaccurate */
    if (*this != 0) {
      FUN_005fc4a0((int *)this);
      FUN_005fc4d0((int *)this);
      return 1;
    }
  }
  return 0;
}

