
undefined4 __thiscall FUN_00589a40(void *this,undefined4 param_1)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  uint uVar4;
  undefined3 extraout_var_00;
  void *pvVar5;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  uint *puVar6;
  uint *puVar7;
  undefined3 extraout_var_04;
  undefined4 uVar8;
  bool bVar9;
  bool bVar10;
  int *piStack_78;
  void *pvStack_74;
  int local_70;
  uint auStack_68 [2];
  uint uStack_60;
  int aiStack_5c [2];
  uint local_54 [2];
  undefined4 local_4c [8];
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fb78;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar3 = FUN_00522480(*(void **)((int)this + 0x14),param_1);
  bVar9 = iVar3 != 0;
  bVar1 = FUN_00520e40(*(int *)((int)this + 0x14));
  if (CONCAT31(extraout_var,bVar1) != 0) {
    *(undefined4 *)((int)this + 0x48) = 1;
  }
  iVar3 = FUN_00520b80(*(int *)((int)this + 0x14));
  if (iVar3 == 0) {
    *(undefined4 *)((int)this + 0x48) = 1;
  }
  uVar4 = FUN_00520e50(*(int *)((int)this + 0x14));
  if (uVar4 == 0) {
    *(undefined4 *)((int)this + 0x48) = 1;
  }
  if (*(int *)((int)this + 0x48) != 0) goto LAB_00589d20;
  uVar4 = *(uint *)(*(int *)((int)this + 0x14) + 0x24) >> 6 & 3;
  if (uVar4 == 3) {
    iVar3 = 3;
  }
  else {
    iVar3 = 2 - (uint)(uVar4 != 1);
  }
  FUN_004ffef0(local_4c,*(undefined4 *)((int)this + 0x10),1,iVar3);
  local_4 = 0;
  bVar1 = FUN_005131b0((int)local_4c);
  local_70 = CONCAT31(extraout_var_00,bVar1);
  uVar4 = *(uint *)(*(int *)((int)this + 0x14) + 0x24) >> 6 & 3;
  if (uVar4 == 3) {
    uVar4 = 3;
  }
  else {
    uVar4 = 2 - (uVar4 != 1);
  }
  bVar10 = (*(uint *)(*(int *)((int)this + 0x10) + 0x24) >> 6 & 3) == uVar4;
  pvVar5 = FUN_005275d0(local_2c,*(int *)((int)this + 0x10),1,
                        *(uint *)(*(int *)((int)this + 0x14) + 0x24) >> 6 & 3);
  local_4._0_1_ = 1;
  bVar1 = FUN_005131b0((int)pvVar5);
  local_4._0_1_ = 0;
  FUN_00527650(local_2c);
  uVar4 = FUN_00520e50(*(int *)((int)this + 0x14));
  switch(uVar4) {
  case 1:
    if (*(int *)((int)this + 0x44) != 0) goto LAB_00589ded;
  case 0:
switchD_00589b7c_caseD_0:
    *(undefined4 *)((int)this + 0x48) = 1;
    break;
  case 2:
    if (*(int *)((int)this + 0x44) == 0) goto switchD_00589b7c_caseD_0;
LAB_00589b8e:
    *(undefined4 *)((int)this + 0x18) = 1;
    if (bVar10) {
      *(undefined4 *)((int)this + 0x20) = 1;
      *(undefined4 *)((int)this + 0x30) = 1;
    }
    break;
  case 3:
    if (*(int *)((int)this + 0x44) == 0) goto LAB_00589b8e;
LAB_00589ded:
    *(undefined4 *)((int)this + 0x1c) = 1;
    bVar10 = FUN_00520bb0(*(int *)((int)this + 0x14));
    if (CONCAT31(extraout_var_04,bVar10) != 0) {
      *(undefined4 *)((int)this + 0x18) = 1;
    }
    if ((local_70 != 0) && (CONCAT31(extraout_var_01,bVar1) == 0)) {
      *(undefined4 *)((int)this + 0x24) = 1;
      *(undefined4 *)((int)this + 0x28) = 1;
      *(undefined4 *)((int)this + 0x2c) = 1;
    }
    break;
  case 4:
    bVar1 = false;
    if (*(int *)((int)this + 0x44) == 0) goto switchD_00589b7c_caseD_0;
    piStack_78 = (int *)0x0;
    pvStack_74 = (void *)0x0;
    bVar2 = FUN_00521160(*(void **)((int)this + 0x14),&piStack_78);
    if ((CONCAT31(extraout_var_02,bVar2) == 0) || (bVar2 = !bVar9, bVar9 = true, bVar2)) {
      bVar9 = false;
    }
    if (piStack_78 != (int *)0x0) {
      bVar2 = FUN_004f6b50(piStack_78,(int *)&pvStack_74);
      if ((CONCAT31(extraout_var_03,bVar2) == 0) || (bVar2 = !bVar9, bVar9 = true, bVar2)) {
        bVar9 = false;
      }
      if ((piStack_78 != (int *)0x0) && (pvStack_74 != (void *)0x0)) {
        puVar6 = FUN_004025b0(piStack_78,&uStack_60);
        local_4._0_1_ = 2;
        puVar7 = (uint *)FUN_0050c640(piStack_78);
        local_4._0_1_ = 3;
        if (*puVar7 == *puVar6) {
LAB_00589ca7:
          bVar1 = true;
        }
        else {
          puVar6 = FUN_004025b0(pvStack_74,auStack_68);
          local_4._0_1_ = 4;
          puVar7 = (uint *)FUN_0050c640(piStack_78);
          bVar1 = *puVar7 == *puVar6;
          FUN_00619730();
          local_4._0_1_ = 3;
          FUN_00619730();
          if (bVar1) goto LAB_00589ca7;
        }
        local_4._0_1_ = 2;
        FUN_00619730();
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
      }
    }
    *(undefined4 *)((int)this + 0x1c) = 1;
    *(undefined4 *)((int)this + 0x18) = 1;
    if (bVar1) {
      if (bVar10) {
        *(undefined4 *)((int)this + 0x20) = 1;
        *(undefined4 *)((int)this + 0x30) = 1;
      }
    }
    else if (local_70 != 0) {
      *(undefined4 *)((int)this + 0x24) = 1;
      *(undefined4 *)((int)this + 0x28) = 1;
      *(undefined4 *)((int)this + 0x2c) = 1;
    }
    break;
  default:
    bVar9 = false;
  }
  local_4 = 0xffffffff;
  FUN_004fff70(local_4c);
LAB_00589d20:
  pvVar5 = (void *)((int)this + 8);
  FUN_00589110(local_54,pvVar5);
  local_4 = 5;
  iVar3 = FUN_005875c0(pvVar5,local_54,param_1);
  if ((iVar3 == 0) || (!bVar9)) {
    bVar9 = false;
  }
  else {
    bVar9 = true;
  }
  FUN_00589000(aiStack_5c,pvVar5);
  local_4._0_1_ = 6;
  iVar3 = FUN_00587b30(pvVar5,aiStack_5c,param_1);
  if ((iVar3 == 0) || (!bVar9)) {
    bVar9 = false;
  }
  else {
    bVar9 = true;
  }
  iVar3 = FUN_005885f0(pvVar5,param_1);
  if ((iVar3 == 0) || (!bVar9)) {
    uVar8 = 0;
  }
  else {
    uVar8 = 1;
  }
  local_4 = CONCAT31(local_4._1_3_,5);
  FUN_00589040(aiStack_5c);
  local_4 = 0xffffffff;
  FUN_00589150(local_54);
  ExceptionList = pvStack_c;
  return uVar8;
}

