
uint __cdecl FUN_00592aa0(int param_1,int *param_2,int *param_3,void *param_4)

{
  void *this;
  uint uVar1;
  uint *puVar2;
  int *piVar3;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_4;
  puStack_8 = &LAB_006508b0;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_4,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  uVar1 = (uint)(param_1 != 3);
  *param_3 = 0;
  if (uVar1 != 0) {
    if (param_1 == 1) {
      param_1 = 1;
    }
    else if (param_1 == 2) {
      param_1 = 2;
    }
    else {
      param_1 = 0;
    }
    FUN_004ece30(&param_4);
    local_4 = 1;
    if ((*(byte *)(param_2 + 0x14) & 4) == 0) {
      piVar3 = (int *)FUN_00504dc0((uint *)(param_2 + 0xe));
      uVar1 = (uint)(piVar3 != (int *)0x0);
      if (uVar1 != 0) {
        uVar1 = (**(code **)(*piVar3 + 0xc))(&param_4);
      }
    }
    else {
      uVar1 = (**(code **)(*param_2 + 0xc))(&param_4);
    }
    puVar2 = FUN_004ece40((uint *)&param_4);
    if (puVar2 == (uint *)0x0) {
      if ((*(byte *)(param_2 + 0x14) & 8) == 0) {
        local_14 = 0x40;
        local_10 = 0x20;
        local_4._0_1_ = 3;
        FUN_00520580(this,&local_14);
      }
      else {
        local_14 = 0x40;
        local_10 = 0x21;
        local_4._0_1_ = 2;
        FUN_00520580(this,&local_14);
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
    }
    if (*(int *)((int)this + 4) == -1) {
      if ((param_2[9] & 0x30U) == 0) {
        piVar3 = (int *)FUN_00504dc0((uint *)&param_4);
      }
      else {
        piVar3 = FUN_004f2d10(param_1,(uint *)&param_4);
      }
      *param_3 = (int)piVar3;
      uVar1 = (uint)(*param_3 != 0);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return uVar1;
}

