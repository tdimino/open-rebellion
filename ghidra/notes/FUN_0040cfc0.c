
uint __thiscall FUN_0040cfc0(void *this,int *param_1,void *param_2)

{
  int *this_00;
  uint uVar1;
  undefined4 *puVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  int iVar3;
  undefined3 extraout_var_05;
  uint uVar4;
  int iVar5;
  int *piVar6;
  undefined4 *puVar7;
  bool bVar8;
  longlong lVar9;
  int *local_34;
  uint local_30;
  undefined4 local_2c [5];
  uint uStack_18;
  uint uStack_14;
  int *local_10;
  uint uStack_c;
  uint uStack_8;
  
  this_00 = param_1;
  bVar8 = *(int *)((int)this + 4) == 0;
  uVar1 = 0;
  if (!bVar8) {
    uVar1 = (uint)(*(int *)((int)this + 8) != 0);
    bVar8 = uVar1 == 0;
  }
  if (!bVar8) {
    uVar1 = FUN_004f6df0(param_1,&local_30);
  }
  if (uVar1 != 0) {
    puVar2 = (undefined4 *)FUN_00596870(local_2c,local_30);
    puVar7 = local_2c;
    for (iVar5 = 0xb; iVar5 != 0; iVar5 = iVar5 + -1) {
      *puVar7 = *puVar2;
      puVar2 = puVar2 + 1;
      puVar7 = puVar7 + 1;
    }
  }
  bVar8 = uVar1 == 0;
  param_1 = (int *)0x0;
  local_34 = (int *)0x0;
  uVar1 = 0;
  if (!bVar8) {
    piVar6 = local_10;
    if (local_10 != (int *)0x0) {
      bVar8 = FUN_00500680(this_00);
      if (CONCAT31(extraout_var,bVar8) != 0) {
        bVar8 = FUN_00500680(this_00);
        param_1 = local_10;
        if (CONCAT31(extraout_var_00,bVar8) < (int)local_10) {
          bVar8 = FUN_00500680(this_00);
          param_1 = (int *)CONCAT31(extraout_var_01,bVar8);
        }
        piVar6 = (int *)((int)local_10 - (int)param_1);
      }
      if ((piVar6 != (int *)0x0) &&
         (bVar8 = FUN_00500690(this_00), CONCAT31(extraout_var_02,bVar8) != 0)) {
        bVar8 = FUN_00500690(this_00);
        local_34 = piVar6;
        if (CONCAT31(extraout_var_03,bVar8) < (int)piVar6) {
          bVar8 = FUN_00500690(this_00);
          local_34 = (int *)CONCAT31(extraout_var_04,bVar8);
        }
        piVar6 = (int *)((int)piVar6 - (int)local_34);
      }
    }
    uVar1 = (uint)(piVar6 == (int *)0x0);
    bVar8 = uVar1 == 0;
  }
  if (!bVar8) {
    iVar5 = (**(code **)(*this_00 + 0x274))();
    iVar3 = (**(code **)(*this_00 + 0x248))();
    lVar9 = __ftol();
    iVar5 = (int)lVar9 - (iVar3 - iVar5);
    if (iVar5 < 1) {
      if (iVar5 < 0) {
        uVar1 = FUN_00501ae0(this_00,-iVar5,0xc,param_2);
      }
    }
    else {
      bVar8 = FUN_00501b70(this_00,iVar5,param_2);
      uVar1 = CONCAT31(extraout_var_05,bVar8);
    }
  }
  uVar4 = 0;
  if (uVar1 != 0) {
    iVar5 = FUN_00501510(this_00,uStack_14,param_2);
    uVar4 = 0;
    if (iVar5 != 0) {
      iVar5 = FUN_005015a0(this_00,uStack_8,param_2);
      uVar4 = 0;
      if (iVar5 != 0) {
        iVar5 = FUN_00501640(this_00,uStack_c,param_2);
        uVar4 = 0;
        if (iVar5 != 0) {
          iVar5 = FUN_005016e0(this_00,uStack_18,param_2);
          uVar4 = 0;
          if (iVar5 != 0) {
            iVar5 = FUN_00501780(this_00,(uint)param_1,param_2);
            uVar4 = 0;
            if (iVar5 != 0) {
              uVar4 = FUN_00501820(this_00,(uint)local_34,param_2);
            }
          }
        }
      }
    }
  }
  return uVar4;
}

