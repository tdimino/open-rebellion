
BOOL __cdecl FUN_00611b00(undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  void *this;
  undefined4 *puVar1;
  int iVar2;
  BOOL BVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656e6b;
  local_c = ExceptionList;
  BVar3 = 0;
  if (DAT_006be6b0 != 0) {
    return 0;
  }
  if (((DAT_006be680 != 0) && (DAT_006be630 != (HANDLE)0x0)) && (DAT_006be6a8 != 0)) {
    ExceptionList = &local_c;
    this = (void *)FUN_00618b70(0x18);
    local_4 = 0;
    if (this == (void *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = FUN_00612960(this,param_1,param_2,param_3);
    }
    local_4 = 0xffffffff;
    EnterCriticalSection((LPCRITICAL_SECTION)&lpCriticalSection_006be690);
    iVar2 = FUN_005f5e40(&DAT_006be5f8,(int)puVar1);
    if (iVar2 == 0) {
      if (puVar1 != (undefined4 *)0x0) {
        (**(code **)*puVar1)(1);
      }
    }
    else {
      BVar3 = SetEvent(DAT_006be630);
    }
    LeaveCriticalSection((LPCRITICAL_SECTION)&lpCriticalSection_006be690);
  }
  ExceptionList = local_c;
  return BVar3;
}

