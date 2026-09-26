
void __thiscall FUN_0040a050(void *this,uint *param_1)

{
  bool bVar1;
  bool bVar2;
  undefined1 uVar3;
  HCURSOR pHVar4;
  char *pcVar5;
  void *pvVar6;
  int iVar7;
  undefined4 uVar8;
  uint *puVar9;
  uint local_b8;
  uint local_b4;
  int local_b0;
  undefined4 local_ac;
  undefined4 *local_a8;
  undefined4 local_a0;
  int local_9c;
  int local_98;
  undefined4 *local_94;
  int local_90;
  undefined4 local_8c;
  undefined4 *local_88;
  undefined4 local_80;
  undefined1 local_7c [4];
  undefined1 local_78 [20];
  int local_64 [22];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  pvVar6 = ExceptionList;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629d9d;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = 0;
  iVar7 = *(int *)((int)this + 8);
  if ((iVar7 < 1) || (0xc < iVar7)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if ((iVar7 < 0x14) || (0x17 < iVar7)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  if ((bVar1) || (bVar2)) {
    local_b8 = 1;
  }
  else {
    local_b8 = 0;
  }
  if (local_b8 == 0) {
    ExceptionList = pvVar6;
    return;
  }
  if (bVar1) {
    local_b8 = (uint)(DAT_006b1218 != (undefined4 *)0x0);
  }
  if (local_b8 == 0) {
    ExceptionList = pvVar6;
    return;
  }
  FUN_004ece30(&local_b4);
  local_90 = *(int *)((int)this + 8);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  local_98 = 1;
  local_9c = 1;
  local_b0 = 1;
  switch(local_90) {
  case 1:
    iVar7 = FUN_0041df80(DAT_006b1218,(int *)&local_b8);
    if (iVar7 == 0) break;
    FUN_00401030();
    goto LAB_0040a622;
  case 2:
    iVar7 = FUN_0041dff0(DAT_006b1218,param_1);
    if (iVar7 != 0) {
      *(undefined4 *)((int)this + 8) = 3;
    }
    break;
  case 3:
    DAT_006b1218[5] = 1;
    *(undefined4 *)((int)this + 8) = 4;
    break;
  case 4:
    local_b8 = FUN_0041e230();
    if ((local_b8 != 0) && (local_b8 = FUN_0041e1d0(&local_b4), local_b8 != 0)) {
      iVar7 = FUN_004ece60(&local_b4);
      *(uint *)((int)this + 8) = (-(uint)(iVar7 != 0) & 0xfffffff9) + 0xc;
    }
    break;
  case 5:
    local_b8 = FUN_0041e1d0(&local_b4);
    if (local_b8 != 0) {
      puVar9 = &local_b4;
      uVar8 = thunk_FUN_004fcee0();
      FUN_0041cfa0(uVar8,puVar9);
    }
    *(undefined4 *)((int)this + 8) = 6;
    break;
  case 6:
    FUN_0041dff0(DAT_006b1218,param_1);
    FUN_0041cfc0(&local_98,&local_b0,(int)this + 0xc);
    if (local_98 != 0) {
      *(uint *)((int)this + 8) = (local_b0 != 0) + 7;
    }
    break;
  case 7:
    if (DAT_006b1218 != (undefined4 *)0x0) {
      FUN_0040afa0(&local_8c);
      local_88 = DAT_006b1218;
      local_4._0_1_ = 1;
      local_80 = 0;
      FUN_004fcd90(local_7c,(undefined4 *)((int)this + 0xc));
      FUN_004fcd90(local_78,(undefined4 *)((int)this + 0x10));
      FUN_0040b0e0(&local_8c);
      local_4._0_1_ = 0;
      FUN_0040b050(&local_8c);
    }
    FUN_005f2f50(&local_ac);
    local_4._0_1_ = 2;
    FUN_00409ec0(&local_ac);
    FUN_006164b0(local_64,1);
    local_4 = CONCAT31(local_4._1_3_,3);
    pcVar5 = (char *)FUN_00583c40((int)&local_ac);
    FUN_005f4b10(local_64,pcVar5);
    iVar7 = FUN_005f49b0();
    if (iVar7 == 0) {
      pHVar4 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f02);
      pHVar4 = SetCursor(pHVar4);
      FUN_00409690(local_64);
      SetCursor(pHVar4);
    }
    FUN_005f4bc0(local_64);
    local_4._0_1_ = 2;
    FUN_004096e0((int)local_64);
    local_4._0_1_ = 0;
    FUN_005f2ff0(&local_ac);
    FUN_0040af40();
    FUN_00596820();
    *(undefined4 *)((int)this + 8) = 0x14;
    break;
  case 8:
    if (DAT_006b1218 != (undefined4 *)0x0) {
      FUN_0040afa0(&local_8c);
      local_4._0_1_ = 8;
      local_88 = DAT_006b1218;
      local_80 = 1;
      FUN_004fcd90(local_7c,(undefined4 *)((int)this + 0xc));
      FUN_004fcd90(local_78,(undefined4 *)((int)this + 0x10));
      FUN_0040b0e0(&local_8c);
      local_4._0_1_ = 0;
      FUN_0040b050(&local_8c);
    }
    FUN_00596820();
    *(undefined4 *)((int)this + 8) = 0x15;
    break;
  case 9:
    local_b8 = FUN_0041e270();
    *(undefined4 *)((int)this + 8) = 10;
    break;
  case 10:
    local_b8 = FUN_0041e1d0(&local_b4);
    if (local_b8 != 0) {
      puVar9 = &local_b4;
      uVar8 = thunk_FUN_004fcee0();
      FUN_0041cff0(uVar8,puVar9);
    }
    *(undefined4 *)((int)this + 8) = 0xb;
    break;
  case 0xb:
    FUN_0041dff0(DAT_006b1218,param_1);
    FUN_0041d010();
    if (local_9c != 0) {
      *(undefined4 *)((int)this + 8) = 4;
    }
    break;
  case 0xc:
    FUN_0041e320((int)DAT_006b1218);
    FUN_0041db60();
LAB_0040a622:
    *(undefined4 *)((int)this + 8) = 2;
    break;
  case 0x14:
    pvVar6 = (void *)FUN_00597530();
    iVar7 = FUN_0059a530(pvVar6);
    if (iVar7 != 0) {
      *(undefined4 *)((int)this + 8) = 0x16;
    }
    break;
  case 0x15:
    pvVar6 = (void *)FUN_00597530();
    iVar7 = FUN_0059a530(pvVar6);
    if (iVar7 != 0) {
      *(undefined4 *)((int)this + 8) = 0x17;
    }
    break;
  case 0x16:
    local_b0 = 0;
    local_94 = (undefined4 *)FUN_0040af60(0x28);
    local_4._0_1_ = 4;
    if (local_94 == (undefined4 *)0x0) {
      DAT_006b1218 = (undefined4 *)0x0;
    }
    else {
      DAT_006b1218 = FUN_0041dce0(local_94);
    }
    local_4._0_1_ = 0;
    uVar3 = (undefined1)local_4;
    local_4._0_1_ = 0;
    if (((DAT_006b1218 != (undefined4 *)0x0) &&
        (uVar3 = (undefined1)local_4, DAT_006b1218 != (undefined4 *)0x0)) && (DAT_006b1218[1] != 0))
    {
      FUN_005f2f50(&local_ac);
      local_4._0_1_ = 5;
      FUN_00409dd0(&local_ac);
      local_4._0_1_ = 0;
      FUN_005f2ff0(&local_ac);
      uVar3 = (undefined1)local_4;
    }
    local_4._0_1_ = uVar3;
    FUN_00409700();
    FUN_005f2f50(&local_ac);
    local_4._0_1_ = 6;
    FUN_00409ec0(&local_ac);
    pcVar5 = (char *)FUN_00583c40((int)&local_ac);
    FUN_005f4bd0(pcVar5);
    local_4._0_1_ = 0;
    FUN_005f2ff0(&local_ac);
    if (DAT_006b1218 != (undefined4 *)0x0) {
      FUN_0040c730(&local_ac);
      local_a8 = DAT_006b1218;
      local_4._0_1_ = 7;
      local_a0 = 0;
      FUN_0040c780(&local_ac,&local_b0);
      local_4._0_1_ = 0;
      FUN_0040c770(&local_ac);
    }
    FUN_005975a0();
    if (DAT_006b1218 != (undefined4 *)0x0) {
      FUN_0041e250();
    }
    *(uint *)((int)this + 8) = 9 - (uint)(local_b0 != 0);
    break;
  case 0x17:
    local_94 = (undefined4 *)0x0;
    if (DAT_006b1218 != (undefined4 *)0x0) {
      FUN_0040c730(&local_ac);
      local_4._0_1_ = 9;
      local_a8 = DAT_006b1218;
      local_a0 = 1;
      FUN_0040c780(&local_ac,(int *)&local_94);
      local_4._0_1_ = 0;
      FUN_0040c770(&local_ac);
    }
    FUN_005975a0();
    if (DAT_006b1218 != (undefined4 *)0x0) {
      FUN_0041e250();
    }
    *(undefined4 *)((int)this + 8) = 9;
  }
  if (*(int *)((int)this + 8) != local_90) {
    FUN_005f2f50(&local_ac);
    local_4._0_1_ = 10;
    FUN_0040a700(*(undefined4 *)((int)this + 8),&local_ac);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f2ff0(&local_ac);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

