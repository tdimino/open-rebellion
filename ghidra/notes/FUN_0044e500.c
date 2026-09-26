
void __thiscall FUN_0044e500(void *this,int param_1,int param_2)

{
  int iVar1;
  int iVar2;
  uint *puVar3;
  void *pvVar4;
  undefined4 *puVar5;
  void *pvVar6;
  void *this_00;
  int *piVar7;
  short sVar8;
  LONG LVar9;
  uint uVar10;
  uint local_88;
  uint local_84;
  int local_80;
  uint *local_7c;
  int local_78;
  int local_74;
  uint local_70;
  undefined4 local_6c;
  undefined4 local_68;
  undefined4 local_64;
  uint local_60;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  void *local_50;
  undefined4 local_4c;
  RECT local_48;
  uint local_38;
  uint local_34;
  undefined4 local_30 [2];
  undefined4 local_28 [2];
  undefined4 local_20 [2];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f8f4;
  local_c = ExceptionList;
  if ((*(int *)((int)this + 0x180) == param_1) && (param_2 == 0)) {
    return;
  }
  ExceptionList = &local_c;
  FUN_00520570(&local_38);
  local_4 = 0;
  local_7c = (uint *)0x0;
  local_80 = 0;
  FUN_004f5510(local_20);
  local_4._0_1_ = 1;
  FUN_004f5510(local_28);
  local_4._0_1_ = 2;
  FUN_004f5510(local_30);
  local_4._0_1_ = 3;
  FUN_005f5b20(*(void **)((int)this + 0x124));
  switch(param_1) {
  case 0x97:
    LVar9 = 3;
    local_84 = 0xa5;
    sVar8 = 0x46;
    local_48.left = 0x14;
    local_48.top = 0x1c;
    local_4._0_1_ = 4;
    FUN_00520580(&local_38,&local_48.left);
    local_4._0_1_ = 3;
    FUN_00619730();
    break;
  case 0x98:
    LVar9 = 3;
    local_84 = 0xa5;
    sVar8 = 0x46;
    local_50 = (void *)0x1c;
    local_4c = 0x20;
    local_4._0_1_ = 5;
    FUN_00520580(&local_38,&local_50);
    local_4._0_1_ = 3;
    FUN_00619730();
    break;
  case 0x99:
    LVar9 = 3;
    local_84 = 0xa5;
    sVar8 = 0x46;
    local_58 = 0x28;
    local_54 = 0x30;
    local_4._0_1_ = 6;
    FUN_00520580(&local_38,&local_58);
    local_4._0_1_ = 3;
    FUN_00619730();
    break;
  case 0x9a:
    LVar9 = 3;
    local_84 = 0xa5;
    sVar8 = 0x46;
    local_60 = 0x22;
    local_5c = 0x28;
    local_4._0_1_ = 7;
    FUN_00520580(&local_38,&local_60);
    local_4._0_1_ = 3;
    FUN_00619730();
    break;
  case 0x9b:
    LVar9 = 3;
    local_84 = 0xa5;
    sVar8 = 0x46;
    local_68 = 0x10;
    local_64 = 0x14;
    local_4._0_1_ = 8;
    FUN_00520580(&local_38,&local_68);
    local_4._0_1_ = 3;
    FUN_00619730();
    break;
  case 0x9c:
    LVar9 = 4;
    local_84 = 0x6e;
    sVar8 = 0x32;
    local_70 = 0x30;
    local_6c = 0x40;
    local_4._0_1_ = 9;
    FUN_00520580(&local_38,&local_70);
    local_4._0_1_ = 3;
    FUN_00619730();
    local_80 = 1;
    local_7c = (uint *)0x1;
    break;
  default:
    sVar8 = (short)local_48.left;
    LVar9 = local_48.left;
  }
  iVar1 = FUN_006076b0(this,LVar9);
  iVar2 = FUN_006073d0(this);
  if (iVar1 == iVar2) {
    local_48.left = 0xc;
    local_48.top = 0x24;
    local_48.right = 0x175;
    local_48.bottom = 0x3b;
    InvalidateRect(*(HWND *)((int)this + 0x18),&local_48,0);
  }
  else {
    puVar3 = (uint *)FUN_006075e0(this,LVar9);
    pvVar4 = *(void **)((int)this + 0x120);
    FUN_00608300(pvVar4,puVar3,*(int *)((int)pvVar4 + 0x28),*(uint *)((int)pvVar4 + 0x2c));
    FUN_006082c0(*(void **)((int)this + 0x120),(int)(short)local_84,(int)sVar8);
  }
  iVar1 = *(int *)(*(int *)((int)this + 0x15c) + 0x94);
  if (iVar1 == 0) {
    iVar1 = 0;
  }
  else {
    iVar1 = *(int *)(iVar1 + 0x24);
  }
  if (iVar1 == 0x66) {
    local_74 = *(int *)((int)this + 0x174);
    iVar1 = local_74 + 0x78;
    iVar2 = local_74 + 0x88;
    local_74 = local_74 + 0x90;
  }
  else {
    if (iVar1 != 0x67) goto LAB_0044ed30;
    local_74 = *(int *)((int)this + 0x174);
    iVar1 = local_74 + 0x70;
    iVar2 = local_74 + 0x80;
    local_74 = local_74 + 0x98;
  }
  pvVar4 = (void *)thunk_FUN_005f5060(iVar1);
  FUN_004ece30(&local_88);
  local_4._0_1_ = 10;
  for (; pvVar4 != (void *)0x0; pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
    puVar5 = FUN_00403040(pvVar4,&local_48);
    local_4._0_1_ = 0xb;
    FUN_004f26d0(&local_88,puVar5);
    local_4._0_1_ = 10;
    FUN_00619730();
    if ((local_38 <= local_88 >> 0x18) && (local_88 >> 0x18 < local_34)) {
      local_50 = (void *)FUN_00618b70(0x1c);
      local_4._0_1_ = 0xc;
      if (local_50 == (void *)0x0) {
        pvVar6 = (void *)0x0;
      }
      else {
        pvVar6 = FUN_004eccd0(local_50,(int)pvVar4);
      }
      local_4._0_1_ = 10;
      FUN_004f57b0(local_20,pvVar6);
    }
  }
  for (pvVar4 = (void *)thunk_FUN_005f5060(iVar2); pvVar4 != (void *)0x0;
      pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
    puVar5 = FUN_00403040(pvVar4,&local_48);
    local_4._0_1_ = 0xd;
    FUN_004f26d0(&local_88,puVar5);
    local_4._0_1_ = 10;
    FUN_00619730();
    if ((local_38 <= local_88 >> 0x18) && (local_88 >> 0x18 < local_34)) {
      local_50 = (void *)FUN_00618b70(0x1c);
      local_4._0_1_ = 0xe;
      if (local_50 == (void *)0x0) {
        pvVar6 = (void *)0x0;
      }
      else {
        pvVar6 = FUN_004eccd0(local_50,(int)pvVar4);
      }
      local_4._0_1_ = 10;
      FUN_004f57b0(local_28,pvVar6);
    }
  }
  if (local_80 != 0) {
    for (pvVar4 = (void *)thunk_FUN_005f5060(local_74); pvVar4 != (void *)0x0;
        pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
      puVar5 = FUN_00403040(pvVar4,&local_48);
      local_4._0_1_ = 0xf;
      FUN_004f26d0(&local_88,puVar5);
      local_4._0_1_ = 10;
      FUN_00619730();
      if ((local_38 <= local_88 >> 0x18) && (local_88 >> 0x18 < local_34)) {
        local_50 = (void *)FUN_00618b70(0x1c);
        local_4._0_1_ = 0x10;
        if (local_50 == (void *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          pvVar6 = FUN_004eccd0(local_50,(int)pvVar4);
        }
        local_4._0_1_ = 10;
        FUN_004f57b0(local_30,pvVar6);
      }
    }
  }
  pvVar4 = (void *)thunk_FUN_005f5060((int)local_20);
  pvVar6 = (void *)thunk_FUN_005f5060((int)local_28);
  if (local_80 == 0) {
    this_00 = (void *)0x0;
  }
  else {
    this_00 = (void *)thunk_FUN_005f5060((int)local_30);
  }
  FUN_005f2f50(local_18);
  local_84 = (uint)(local_80 == 0);
  local_4._0_1_ = 0x11;
  local_74 = 0;
  local_78 = 0;
  while (((local_74 == 0 || (local_78 == 0)) || (local_84 == 0))) {
    if (pvVar4 == (void *)0x0) {
      if (local_74 == 0) {
        local_74 = 1;
        local_68 = CONCAT22(DAT_0065d424,0x8526);
        FUN_005f3010(local_18,&local_68);
        puVar5 = local_18;
      }
      else {
        puVar5 = (undefined4 *)0x0;
      }
      FUN_0044edb0(this,(int *)0x0,(uint *)0x0,(uint)puVar5);
    }
    else {
      FUN_00403040(pvVar4,&local_70);
      uVar10 = 0;
      local_4._0_1_ = 0x12;
      puVar3 = local_7c;
      piVar7 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x118) + 0x9c),&local_70);
      FUN_0044edb0(this,piVar7,puVar3,uVar10);
      pvVar4 = *(void **)((int)pvVar4 + 0x10);
      if (pvVar4 == (void *)0x0) {
        local_74 = 1;
      }
      local_4._0_1_ = 0x11;
      FUN_00619730();
    }
    if (local_80 != 0) {
      if (this_00 == (void *)0x0) {
        if (local_84 == 0) {
          local_84 = 1;
          local_58 = CONCAT22(DAT_0065d424,0x8526);
          FUN_005f3010(local_18,&local_58);
          puVar5 = local_18;
        }
        else {
          puVar5 = (undefined4 *)0x0;
        }
        FUN_0044edb0(this,(int *)0x0,(uint *)0x0,(uint)puVar5);
      }
      else {
        FUN_00403040(this_00,&local_60);
        uVar10 = 0;
        local_4._0_1_ = 0x13;
        puVar3 = local_7c;
        piVar7 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x118) + 0x9c),&local_60);
        FUN_0044edb0(this,piVar7,puVar3,uVar10);
        this_00 = *(void **)((int)this_00 + 0x10);
        if (this_00 == (void *)0x0) {
          local_84 = 1;
        }
        local_4._0_1_ = 0x11;
        FUN_00619730();
      }
    }
    if (pvVar6 == (void *)0x0) {
      if (local_78 == 0) {
        local_78 = 1;
        local_48.left = CONCAT22(DAT_0065d424,0x1941);
        FUN_005f3010(local_18,&local_48);
        FUN_0044edb0(this,(int *)0x0,(uint *)0x0,(uint)local_18);
      }
      else {
        FUN_0044edb0(this,(int *)0x0,(uint *)0x0,0);
      }
    }
    else {
      FUN_00403040(pvVar6,&local_50);
      uVar10 = 0;
      local_4._0_1_ = 0x14;
      puVar3 = local_7c;
      piVar7 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x118) + 0x9c),(uint *)&local_50);
      FUN_0044edb0(this,piVar7,puVar3,uVar10);
      pvVar6 = *(void **)((int)pvVar6 + 0x10);
      if (pvVar6 == (void *)0x0) {
        local_78 = 1;
      }
      local_4._0_1_ = 0x11;
      FUN_00619730();
    }
  }
  FUN_0060a280(*(void **)((int)this + 0x120));
  local_48.left = *(int *)((int)this + 0x13c);
  local_48.top = *(LONG *)((int)this + 0x140);
  local_48.right = *(LONG *)((int)this + 0x144);
  local_48.bottom = *(LONG *)((int)this + 0x148);
  InvalidateRect(*(HWND *)((int)this + 0x18),&local_48,0);
  local_4._0_1_ = 10;
  *(int *)((int)this + 0x180) = param_1;
  FUN_005f2ff0(local_18);
  local_4._0_1_ = 3;
  FUN_00619730();
LAB_0044ed30:
  local_4._0_1_ = 2;
  FUN_004f55c0(local_30);
  local_4._0_1_ = 1;
  FUN_004f55c0(local_28);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f55c0(local_20);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

