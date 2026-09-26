
undefined4 __cdecl FUN_00517cd0(undefined4 param_1,int param_2,uint *param_3,void *param_4)

{
  bool bVar1;
  int iVar2;
  int *piVar3;
  uint uVar4;
  undefined3 extraout_var;
  undefined4 uVar5;
  bool bVar6;
  uint local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006429e0;
  local_c = ExceptionList;
  uVar5 = 1;
  if (param_2 == 0) {
    ExceptionList = &local_c;
    FUN_00562ae0(local_28);
    local_4 = 0;
    iVar2 = FUN_0055e1f0(local_28);
    bVar6 = iVar2 != 0;
    piVar3 = (int *)FUN_00504dc0(param_3);
    if (piVar3 != (int *)0x0) {
      local_4._0_1_ = 1;
      uVar4 = (**(code **)(*piVar3 + 4))();
      if ((uVar4 < 0x90) || (0x97 < uVar4)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      if (bVar1) {
        bVar1 = FUN_00518ac0(piVar3,local_28,param_4);
        if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar6)) {
          bVar6 = false;
        }
        else {
          bVar6 = true;
        }
      }
    }
    iVar2 = FUN_005180e0(param_4);
    if ((iVar2 == 0) || (!bVar6)) {
      uVar5 = 0;
    }
    else {
      uVar5 = 1;
    }
    local_4 = 0xffffffff;
    FUN_00562b30();
  }
  ExceptionList = local_c;
  return uVar5;
}

