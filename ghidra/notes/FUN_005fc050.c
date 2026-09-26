
undefined4 __thiscall FUN_005fc050(void *this,uint param_1)

{
  HRSRC hResInfo;
  HGLOBAL pvVar1;
  LPVOID pvVar2;
  
  if (*(HMODULE *)((int)this + 0x18) != (HMODULE)0x0) {
    if (*(int *)((int)this + 0x14) != 0) {
      *(undefined4 *)((int)this + 0x14) = 0;
    }
    hResInfo = FindResourceA(*(HMODULE *)((int)this + 0x18),(LPCSTR)(param_1 & 0xffff),&DAT_00000002
                            );
    if (hResInfo == (HRSRC)0x0) {
      GetLastError();
    }
    pvVar1 = LoadResource(*(HMODULE *)((int)this + 0x18),hResInfo);
    *(HGLOBAL *)((int)this + 0x14) = pvVar1;
    if (pvVar1 == (HGLOBAL)0x0) {
      GetLastError();
    }
    if (*(HGLOBAL *)((int)this + 0x14) != (HGLOBAL)0x0) {
      pvVar2 = LockResource(*(HGLOBAL *)((int)this + 0x14));
      *(LPVOID *)this = pvVar2;
    }
    if (*(int *)this != 0) {
      FUN_005fc4a0(this);
      FUN_005fc4d0(this);
      FUN_005fd170(this);
      return 1;
    }
  }
  return 0;
}

