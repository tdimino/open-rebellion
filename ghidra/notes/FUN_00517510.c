
uint __cdecl FUN_00517510(int *param_1,void *param_2)

{
  bool bVar1;
  void *pvVar2;
  undefined3 extraout_var;
  int iVar3;
  uint uVar4;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006428e8;
  local_c = ExceptionList;
  uVar4 = 1;
  if (((param_1[0x14] & 8U) != 0) && ((param_1[0x14] & 0x4000U) == 0)) {
    ExceptionList = &local_c;
    pvVar2 = FUN_004fcd00(local_2c,param_1,4);
    local_4 = 0;
    bVar1 = FUN_005131b0((int)pvVar2);
    local_4 = 0xffffffff;
    FUN_004fcd80(local_2c);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      iVar3 = (**(code **)(*param_1 + 0x40))();
      if (iVar3 == 0) {
        uVar4 = FUN_0053fbd0(0x372,param_1,param_2);
      }
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

