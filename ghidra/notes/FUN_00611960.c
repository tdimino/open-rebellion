
undefined4 __cdecl FUN_00611960(int param_1,LPCSTR param_2,int param_3,undefined4 *param_4)

{
  undefined4 uVar1;
  void *pvVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656e4b;
  local_c = ExceptionList;
  uVar1 = 1;
  if (param_2 != (LPCSTR)0x0) {
    if ((*(uint *)(param_1 + 0x10) & 7) != 1) {
      return 3;
    }
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x58);
    local_4 = 0;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_00612280(pvVar2,param_2,param_3,1,param_1);
    }
    *param_4 = pvVar2;
    uVar1 = 0;
  }
  ExceptionList = local_c;
  return uVar1;
}

