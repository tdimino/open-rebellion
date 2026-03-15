
int FUN_0055f320(uint *param_1,undefined4 param_2,void *param_3)

{
  int *piVar1;
  uint uVar2;
  int iVar3;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649fc0;
  local_c = ExceptionList;
  iVar3 = 1;
  local_14 = 1;
  local_10 = 1;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar1 = (int *)FUN_00504dc0(param_1);
  if (piVar1 != (int *)0x0) {
    local_14 = 0x30;
    local_10 = 0x3c;
    local_4 = 1;
    uVar2 = (**(code **)(*piVar1 + 4))();
    if ((uVar2 < local_14) || (local_10 <= uVar2)) {
      iVar3 = 0;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar3 != 0) {
      iVar3 = (**(code **)(*piVar1 + 0x2d4))(param_2,param_3);
    }
  }
  ExceptionList = local_c;
  return iVar3;
}

