
int FUN_0054b040(int *param_1,void *param_2)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  uint uVar4;
  uint uVar5;
  uint *puVar6;
  void *pvVar7;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar8;
  undefined3 extraout_var_01;
  int *unaff_EBP;
  int *unaff_ESI;
  void *unaff_retaddr;
  int *local_60;
  void *local_5c;
  uint uStack_58;
  int local_54;
  void *local_50;
  uint local_4c;
  uint local_48;
  uint local_44;
  int local_40;
  int local_3c [3];
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined1 uStack_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647f48;
  local_c = ExceptionList;
  if (param_1 == (int *)0x0) {
    return 1;
  }
  param_1 = (int *)0x0;
  local_60 = (int *)0x0;
  local_5c = (void *)0x0;
  local_40 = 0;
  local_3c[0] = 0;
  local_3c[1] = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_44);
  local_4 = 0;
  FUN_004ece30(&local_48);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_4c);
  local_4 = CONCAT31(local_4._1_3_,2);
  local_54 = 0;
  uVar4 = FUN_005434b0(&param_1,&local_40,&local_44);
  uVar5 = FUN_00543760(&local_60,local_3c,&local_48);
  if ((uVar5 == 0) || (uVar4 == 0)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  uVar4 = FUN_00543800(&local_5c,local_3c + 1,&local_4c);
  if ((uVar4 == 0) || (!bVar2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  puVar6 = FUN_004ece40(&local_44);
  if (((puVar6 != (uint *)0x0) && (puVar6 = FUN_004ece40(&local_48), puVar6 != (uint *)0x0)) &&
     (puVar6 = FUN_004ece40(&local_4c), puVar6 != (uint *)0x0)) {
    if (((local_44 == local_4c) && (local_48 == local_4c)) && (bVar2)) {
      bVar2 = true;
    }
    else {
      bVar2 = false;
    }
  }
  if (((bVar2) && (param_1 != (int *)0x0)) &&
     ((local_60 != (int *)0x0 && (local_5c != (void *)0x0)))) {
    puVar6 = FUN_0050c640(param_1);
    local_30 = 0x90;
    local_2c = 0x98;
    local_4._0_1_ = 3;
    if ((*puVar6 >> 0x18 < 0x90) || (0x97 < *puVar6 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    local_4._0_1_ = 2;
    FUN_00619730();
    bVar3 = true;
    if (bVar2) {
      puVar6 = FUN_0050c640(param_1);
      local_4._0_1_ = 4;
      pvVar7 = (void *)FUN_00505f60(puVar6);
      local_4._0_1_ = 2;
      FUN_00619730();
      if (pvVar7 != (void *)0x0) {
        bVar2 = FUN_0050d5a0(pvVar7,6,1,(int)param_2);
        bVar3 = FUN_0050d5a0(pvVar7,6,2,(int)param_2);
        if ((CONCAT31(extraout_var_00,bVar3) == 0) || (CONCAT31(extraout_var,bVar2) == 0)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
      }
    }
    pvVar7 = param_2;
    FUN_004fd450(&local_28,(int)param_2);
    local_4._0_1_ = 5;
    local_24 = 1;
    local_20 = 3;
    iVar8 = FUN_004f9510(param_1,1,&local_28);
    if ((iVar8 == 0) || (!bVar3)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar8 = FUN_004f9510(param_1,2,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar8 = FUN_004f9510(local_60,1,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar8 = FUN_004f9510(local_60,2,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar8 = FUN_004f9510(local_5c,1,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar8 = FUN_004f9510(local_5c,2,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    if ((short)param_1[0x23] < DAT_006bb564) {
      local_54 = 2;
      iVar8 = FUN_004eeb10(param_1,0,pvVar7);
      if ((iVar8 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      iVar8 = FUN_00549a20(pvVar7);
      if ((iVar8 == 0) || (!bVar2)) goto LAB_0054b465;
      bVar2 = true;
    }
    else {
      local_54 = 1;
      iVar8 = (**(code **)(*param_1 + 0x218))(pvVar7);
      if ((iVar8 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      puVar6 = FUN_004025b0(unaff_retaddr,(uint *)&local_5c);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,6);
      iVar8 = (**(code **)(*unaff_EBP + 0x20c))(puVar6,pvVar7);
      if ((iVar8 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      uStack_10 = 5;
      FUN_00619730();
      puVar6 = FUN_004025b0(puStack_8,(uint *)&stack0xffffff9c);
      uStack_10 = 7;
      iVar8 = (**(code **)(*unaff_ESI + 0x20c))(puVar6,pvVar7);
      if ((iVar8 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      local_4._0_1_ = 5;
      FUN_00619730();
      iVar8 = FUN_004eeb10(local_60,0,pvVar7);
      if ((iVar8 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      iVar8 = FUN_004eeb10(local_5c,0,pvVar7);
      if ((iVar8 == 0) || (!bVar2)) {
LAB_0054b465:
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
    }
    iVar8 = FUN_004f9510(param_1,1,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar8 = FUN_004f9510(param_1,2,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar8 = FUN_004f9510(local_60,1,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar8 = FUN_004f9510(local_60,2,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar8 = FUN_004f9510(local_5c,1,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar8 = FUN_004f9510(local_5c,2,&local_28);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_004fd4d0(&local_28);
  }
  iVar8 = 0;
  if (bVar2) {
    if ((((param_1[9] ^ *(uint *)((int)local_5c + 0x24)) & 0xc0) == 0) &&
       (((local_60[9] ^ *(uint *)((int)local_5c + 0x24)) & 0xc0) == 0)) {
      iVar8 = 1;
    }
    else {
      iVar8 = 0;
    }
  }
  bVar2 = false;
  if ((((param_1 != (int *)0x0) && (local_40 != 0)) && (local_60 != (int *)0x0)) &&
     (local_3c[0] != 0)) {
    if (local_54 == 1) {
      uVar4 = *(uint *)(local_40 + 0x24) ^ param_1[9];
    }
    else {
      uVar4 = *(uint *)(local_3c[0] + 0x24) ^ local_60[9];
    }
    bVar2 = (uVar4 & 0xc0) != 0;
  }
  if (!bVar2) goto LAB_0054b6f0;
  FUN_004f4340(&local_30);
  local_4._0_1_ = 8;
  puVar6 = FUN_004025b0(param_1,(uint *)(local_3c + 2));
  local_4._0_1_ = 9;
  FUN_004f44b0(&local_30,puVar6,0);
  local_4._0_1_ = 8;
  FUN_00619730();
  puVar6 = FUN_004025b0(local_60,(uint *)(local_3c + 2));
  local_4._0_1_ = 10;
  FUN_004f44b0(&local_30,puVar6,0);
  local_4._0_1_ = 8;
  FUN_00619730();
  puVar6 = FUN_004025b0(local_5c,(uint *)(local_3c + 2));
  local_4._0_1_ = 0xb;
  FUN_004f44b0(&local_30,puVar6,0);
  local_4._0_1_ = 8;
  FUN_00619730();
  FUN_004ece30(&uStack_58);
  iVar1 = local_54;
  local_4 = CONCAT31(local_4._1_3_,0xc);
  if (local_54 == 1) {
    uVar4 = FUN_00543040(param_1,(uint *)0x0,&uStack_58);
    if ((uVar4 == 0) || (iVar8 == 0)) {
LAB_0054b69c:
      iVar8 = 0;
    }
    else {
      iVar8 = 1;
    }
  }
  else {
    uVar4 = FUN_00543040(local_60,(uint *)0x0,&uStack_58);
    if ((uVar4 == 0) || (iVar8 == 0)) goto LAB_0054b69c;
    iVar8 = 1;
  }
  puVar6 = FUN_004ece40(&uStack_58);
  if (puVar6 != (uint *)0x0) {
    bVar2 = FUN_00547480(local_50,iVar1,&uStack_58,(int)&local_30,(uint)param_2);
    if ((CONCAT31(extraout_var_01,bVar2) == 0) || (iVar8 == 0)) {
      iVar8 = 0;
    }
    else {
      iVar8 = 1;
    }
  }
  local_4._0_1_ = 8;
  FUN_00619730();
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_004f4380(&local_30);
LAB_0054b6f0:
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar8;
}

