
uint __thiscall FUN_0040d320(void *this,int *param_1,undefined4 param_2)

{
  int *this_00;
  uint uVar1;
  int *piVar2;
  uint *puVar3;
  int iVar4;
  void *pvVar5;
  int *piVar6;
  bool bVar7;
  uint local_48;
  int *local_44;
  void *local_40;
  uint local_3c;
  int local_38 [4];
  uint local_28;
  int local_24;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a270;
  local_c = ExceptionList;
  uVar1 = (uint)(*(int *)((int)this + 4) != 0);
  bVar7 = uVar1 == 0;
  if (!bVar7) {
    uVar1 = (uint)(*(int *)((int)this + 8) != 0);
    bVar7 = uVar1 == 0;
  }
  ExceptionList = &local_c;
  local_40 = this;
  if (!bVar7) {
    ExceptionList = &local_c;
    uVar1 = FUN_004f6df0(param_1,&local_3c);
  }
  if (uVar1 != 0) {
    piVar2 = (int *)FUN_005968e0(local_38,local_3c);
    pvVar5 = local_40;
    piVar6 = local_38;
    for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 + -1) {
      *piVar6 = *piVar2;
      piVar2 = piVar2 + 1;
      piVar6 = piVar6 + 1;
    }
    if (uVar1 != 0) {
      FUN_004ece30(&local_44);
      local_4 = 0;
      FUN_004ece30(&param_1);
      local_4 = CONCAT31(local_4._1_3_,1);
      uVar1 = FUN_004f6b70(this_00,(uint *)&local_44);
      if (uVar1 != 0) {
        if (local_24 != 0) {
          piVar2 = (int *)FUN_00596870(local_38,local_28);
          piVar6 = local_38;
          for (iVar4 = 0xb; iVar4 != 0; iVar4 = iVar4 + -1) {
            *piVar6 = *piVar2;
            piVar2 = piVar2 + 1;
            piVar6 = piVar6 + 1;
          }
          local_48 = local_48 & 0xffffff | local_38[2] << 0x18;
          local_48 = local_48 ^ (local_48 ^ local_38[3]) & 0xffffff;
          local_4 = CONCAT31(local_4._1_3_,2);
          iVar4 = FUN_00504f70(&local_48);
          if ((iVar4 != 0) && (((*(uint *)(iVar4 + 0x24) ^ this_00[9]) & 0xc0) == 0)) {
            FUN_004f26d0(&param_1,&local_48);
          }
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00619730();
          pvVar5 = local_40;
        }
        iVar4 = FUN_004ece60((uint *)&param_1);
        if (iVar4 == 0) {
          puVar3 = FUN_004025b0(*(void **)((int)pvVar5 + 8),(uint *)&local_40);
          local_4._0_1_ = 3;
          FUN_004f26d0(&param_1,puVar3);
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00619730();
        }
        if ((uVar1 != 0) && (local_44 != param_1)) {
          uVar1 = (**(code **)(*this_00 + 0xa8))(&param_1,param_2);
        }
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return uVar1;
}

