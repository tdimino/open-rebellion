
undefined4 __fastcall FUN_006010e0(int param_1)

{
  HINSTANCE hmod;
  DWORD dwThreadId;
  void *pvVar1;
  void *this;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006560b1;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (DAT_006be4e4 == 0) {
    hmod = *(HINSTANCE *)(param_1 + 0x1c);
    ExceptionList = &pvStack_c;
    dwThreadId = GetCurrentThreadId();
    DAT_006be5b0 = SetWindowsHookExA(7,lpfn_00600ef0,hmod,dwThreadId);
  }
  pvVar1 = (void *)FUN_00618b70(0x20);
  uStack_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_0060f180(pvVar1,*(undefined4 *)(param_1 + 0x18));
  }
  uStack_4 = 0xffffffff;
  *(int *)((int)pvVar1 + 0x1c) = param_1;
  this = (void *)FUN_00618b70(0x10);
  uStack_4 = 1;
  if (this == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_0060f100(this,pvVar1);
  }
  uStack_4 = 0xffffffff;
  FUN_005f6700(&DAT_006be4e0,(int)pvVar1);
  pvVar1 = (void *)FUN_00618b70(0x20);
  uStack_4 = 2;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_0060f180(pvVar1,*(undefined4 *)(param_1 + 0x18));
  }
  uStack_4 = 0xffffffff;
  FUN_005f4f10((void *)(param_1 + 0x88),(int)pvVar1);
  FUN_006010a0();
  ExceptionList = pvStack_c;
  return 1;
}

