
void __thiscall FUN_004a7f50(void *this,uint *param_1,int *param_2)

{
  bool bVar1;
  int *piVar2;
  short sVar3;
  int iVar4;
  uint *puVar5;
  int iVar6;
  undefined4 *puVar7;
  uint *this_00;
  undefined *puVar8;
  uint uVar9;
  char *pcVar10;
  void *pvVar11;
  short sVar12;
  uint local_60;
  int local_5c;
  int *local_58;
  uint local_54;
  void *local_50;
  RECT local_4c;
  undefined4 local_3c;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638038;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_58 = FUN_004a25c0(this);
  if (local_58 == (int *)0x0) {
    ExceptionList = local_c;
    return;
  }
  local_50 = *(void **)((int)this + 0x174);
  local_54 = *(uint *)((int)this + 0x158);
  local_5c = 0;
  local_60 = 0;
  FUN_004ece30(&local_3c);
  piVar2 = param_2;
  puVar5 = param_1;
  local_4 = 0;
  local_4c.left = 0x90;
  local_4c.top = 0x98;
  if (((*param_1 >> 0x18 < 0x90) || (0x97 < *param_1 >> 0x18)) ||
     (*(uint *)((int)this + 0x148) == ((uint)param_2[9] >> 6 & 3))) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    *(uint *)((int)this + 0x148) = (uint)piVar2[9] >> 6 & 3;
    FUN_004a9ce0(this);
    uVar9 = *(uint *)((int)this + 0x158);
    *(undefined4 *)((int)this + 0x158) = 0;
    FUN_0060d7e0(*(void **)((int)this + 0x160),uVar9,1);
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
    puVar5 = param_1;
    goto LAB_004a8491;
  }
  local_38 = 0x22;
  local_34 = 0x23;
  uVar9 = *puVar5 >> 0x18;
  local_4 = CONCAT31(local_4._1_3_,1);
  if ((uVar9 < 0x22) || (0x22 < uVar9)) {
    local_4c.left = 0x23;
    local_4c.top = 0x24;
    if ((uVar9 < 0x23) || (0x23 < uVar9)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) goto LAB_004a809e;
    bVar1 = false;
  }
  else {
LAB_004a809e:
    bVar1 = true;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  piVar2 = local_58;
  if (bVar1) {
    if (*(int *)((int)this + 0x158) == 5) {
      local_5c = 1;
    }
    iVar6 = FUN_005273d0(local_58,*(undefined4 *)((int)this + 0x148),3);
    iVar4 = FUN_00527150(piVar2,*(undefined4 *)((int)this + 0x148),3);
    local_50 = *(void **)((int)this + 0x174);
    local_54 = 5;
    local_60 = (-(uint)(iVar6 + iVar4 != 0) & 0xfffffffe) + 0x2938;
    puVar5 = param_1;
    goto LAB_004a8491;
  }
  local_4c.left = 0x24;
  local_4c.top = 0x25;
  uVar9 = *param_1 >> 0x18;
  local_4 = CONCAT31(local_4._1_3_,2);
  if ((uVar9 < 0x24) || (0x24 < uVar9)) {
    local_38 = 0x25;
    local_34 = 0x26;
    if ((uVar9 < 0x25) || (0x25 < uVar9)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) goto LAB_004a816b;
    bVar1 = false;
  }
  else {
LAB_004a816b:
    bVar1 = true;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  piVar2 = local_58;
  puVar5 = param_1;
  if (bVar1) {
    if (*(int *)((int)this + 0x158) == 4) {
      local_5c = 1;
    }
    iVar6 = FUN_0051c0d0(local_58,*(undefined4 *)((int)this + 0x148),3);
    iVar4 = FUN_005276d0(piVar2,*(undefined4 *)((int)this + 0x148),3);
    local_50 = *(void **)((int)this + 0x174);
    local_54 = 4;
    local_60 = (-(uint)(iVar6 + iVar4 != 0) & 0xfffffffe) + 0x293b;
    puVar5 = param_1;
  }
  else {
    local_38 = 0x1c;
    local_34 = 0x20;
    if ((*param_1 >> 0x18 < 0x1c) || (0x1f < *param_1 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (*(int *)((int)this + 0x158) == 3) {
        local_5c = 1;
      }
      iVar6 = FUN_00503550(local_58,*(undefined4 *)((int)this + 0x148),3);
      if (*(int *)((int)this + 0x148) == 1) {
        local_54 = 3;
        local_60 = (-(uint)(iVar6 != 0) & 0xfffffffe) + 0x293e;
      }
      else if (*(int *)((int)this + 0x148) == 2) {
        local_54 = 3;
        local_60 = (-(uint)(iVar6 != 0) & 0xfffffffe) + 0x2941;
      }
      else {
        local_60 = 0x2942;
        local_54 = 3;
      }
    }
    else {
      local_38 = 0x10;
      local_34 = 0x14;
      if ((*puVar5 >> 0x18 < 0x10) || (0x13 < *puVar5 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        if (*(int *)((int)this + 0x158) == 2) {
          local_5c = 1;
        }
        iVar6 = FUN_005044f0(local_58,*(undefined4 *)((int)this + 0x148),3);
        if (*(int *)((int)this + 0x148) == 1) {
          local_54 = 2;
          local_60 = (-(uint)(iVar6 != 0) & 0xfffffffe) + 0x2945;
        }
        else if (*(int *)((int)this + 0x148) == 2) {
          local_54 = 2;
          local_60 = (-(uint)(iVar6 != 0) & 0xfffffffe) + 0x2948;
        }
        else {
          local_60 = 0x2949;
          local_54 = 2;
        }
      }
      else {
        local_38 = 0x30;
        local_34 = 0x40;
        if ((*puVar5 >> 0x18 < 0x30) || (0x3f < *puVar5 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          FUN_004ece30(&local_4c.left);
          local_4._0_1_ = 3;
          if (*(int *)((int)this + 0x158) == 1) {
            local_5c = 1;
          }
          local_54 = 0;
          FUN_00536e20(local_2c,local_58,3,*(undefined4 *)((int)this + 0x148));
          local_4 = CONCAT31(local_4._1_3_,4);
          FUN_00513120((int)local_2c);
          uVar9 = 0;
          while (local_10 != 0) {
            pvVar11 = (void *)FUN_0052bed0((int)local_2c);
            puVar5 = FUN_0042d170(pvVar11,&local_60);
            local_4._0_1_ = 5;
            iVar6 = FUN_004ece60(puVar5);
            if ((iVar6 == 0) || ((*(uint *)((int)pvVar11 + 0x78) & 0x100) != 0)) {
              bVar1 = true;
            }
            else {
              bVar1 = false;
            }
            local_4 = CONCAT31(local_4._1_3_,4);
            FUN_00619730();
            if (bVar1) {
              local_54 = local_54 + 1;
            }
            FUN_005130d0((int)local_2c);
            uVar9 = local_54;
          }
          if (*(int *)((int)this + 0x148) == 1) {
            local_60 = (-(uint)(uVar9 != 0) & 0xfffffffe) + 0x294c;
          }
          else if (*(int *)((int)this + 0x148) == 2) {
            local_60 = (-(uint)(uVar9 != 0) & 0xfffffffe) + 0x294f;
          }
          else {
            local_60 = 0x2950;
          }
          local_54 = 1;
          local_4._1_3_ = (uint3)(local_4 >> 8);
          local_4._0_1_ = 3;
          FUN_00536ea0(local_2c);
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
          puVar5 = param_1;
        }
      }
    }
  }
LAB_004a8491:
  if (local_5c != 0) {
    iVar6 = FUN_0060a860(*(void **)((int)this + 0x168),*puVar5 & 0xffffff);
    if (iVar6 == 0) {
      FUN_004a9ab0(this,param_2,local_50);
    }
    else {
      pvVar11 = FUN_004f5940((void *)((int)this + 0x114),puVar5);
      if (pvVar11 == (void *)0x0) {
        puVar7 = (undefined4 *)FUN_0060a860(*(void **)((int)this + 0x168),*puVar5 & 0xffffff);
        FUN_005f5ac0(*(void **)((int)this + 0x168),(int)puVar7);
        if (puVar7 != (undefined4 *)0x0) {
          (**(code **)*puVar7)(1);
        }
      }
      else {
        local_4c.left = *(undefined4 *)(iVar6 + 0x20);
        puVar5 = *(uint **)(iVar6 + 0x24);
        this_00 = FUN_0042c3b0(*(undefined4 *)((int)this + 0x15c),param_1,(int *)0x0,0,1);
        FUN_005fcc30(this_00,(uint *)local_4c.left,0,0,0,0,(undefined4 *)0x0,0);
        FUN_005fcc30(this_00,puVar5,0,0,0,0,(undefined4 *)0x0,0);
        puVar8 = FUN_004f62d0((int)param_2);
        pcVar10 = (char *)FUN_00583c40((int)puVar8);
        FUN_005f35e0((void *)(iVar6 + 0x14),pcVar10);
        FUN_005fd0f0(local_50,puVar5,0,0);
        if (this_00 != (uint *)0x0) {
          FUN_005fbfa0(this_00);
          FUN_00618b60((undefined *)this_00);
        }
      }
    }
    FUN_0060a280(*(void **)((int)this + 0x164));
  }
  if ((*(int *)((int)this + 0x158) == 2) &&
     (uVar9 = FUN_0041cdb0(), piVar2 = local_58, ((uint)local_58[9] >> 6 & 3) == uVar9)) {
    param_1 = (uint *)CONCAT22(DAT_0065d424,0x1947);
    FUN_005f2fc0(&local_38,&param_1);
    local_4._0_1_ = 6;
    FUN_005f31a0(&local_38,piVar2[0x20]);
    pcVar10 = (char *)FUN_00583c40((int)&local_38);
    FUN_00601aa0(*(void **)((int)this + 0x170),pcVar10);
    iVar6 = *(int *)((int)this + 0x170);
    local_4c.left = *(undefined4 *)(iVar6 + 0x14);
    local_4c.top = *(LONG *)(iVar6 + 0x18);
    local_4c.right = *(LONG *)(iVar6 + 0x1c);
    local_4c.bottom = *(LONG *)(iVar6 + 0x20);
    InvalidateRect(*(HWND *)((int)this + 0x18),&local_4c,0);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f2ff0(&local_38);
  }
  uVar9 = local_60;
  if ((local_54 != 0) && (sVar12 = (short)local_60, sVar12 != 0)) {
    pvVar11 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),local_54);
    sVar3 = FUN_006034a0(pvVar11,0);
    if (sVar12 != sVar3) {
      FUN_00603150(pvVar11,0,uVar9);
      InvalidateRect(*(HWND *)((int)pvVar11 + 0x18),(RECT *)0x0,0);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

