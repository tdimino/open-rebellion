
void __thiscall FUN_00454b30(void *this,uint *param_1)

{
  uint uVar1;
  uint uVar2;
  bool bVar3;
  ushort uVar4;
  undefined4 uVar5;
  int *piVar6;
  undefined4 *puVar7;
  void *pvVar8;
  void *this_00;
  int iVar9;
  uint *puVar10;
  uint *puVar11;
  void *local_34;
  uint local_30;
  RECT local_28;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ffc6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar5 = FUN_006037f0(7);
  bVar3 = false;
  local_34 = (void *)0x0;
  local_30 = 0;
  FUN_005f2f50(local_18);
  local_4 = 0;
  piVar6 = FUN_004a25c0(this);
  puVar11 = param_1;
  if (piVar6 != (int *)0x0) {
    local_28.left = 0x2c;
    local_28.top = 0x2d;
    if ((0x2b < *param_1 >> 0x18) && (*param_1 >> 0x18 < 0x2d)) {
      bVar3 = true;
    }
    FUN_00619730();
    if (bVar3) {
      puVar7 = (undefined4 *)FUN_0060a860(*(void **)((int)this + 0x178),*puVar11 & 0xffffff);
      if (puVar7 != (undefined4 *)0x0) {
        FUN_004ece30(&param_1);
        local_4._0_1_ = 1;
        FUN_005f5ac0(*(void **)((int)this + 0x178),(int)puVar7);
        (**(code **)*puVar7)(1);
        local_28.left = FUN_00618b70(0x28);
        local_4._0_1_ = 2;
        if ((void *)local_28.left == (void *)0x0) {
          pvVar8 = (void *)0x0;
        }
        else {
          pvVar8 = FUN_005fbd20((void *)local_28.left,uVar5,0x232d,10);
        }
        local_4._0_1_ = 1;
        local_28.left = FUN_00618b70(0x74);
        local_4._0_1_ = 3;
        if ((void *)local_28.left == (void *)0x0) {
          this_00 = (void *)0x0;
        }
        else {
          this_00 = FUN_004421d0((void *)local_28.left,(uint *)&param_1);
        }
        local_4._0_1_ = 1;
        FUN_0060be60(this_00,(int)pvVar8,0);
        *(uint *)((int)this_00 + 0x3c) = *(uint *)((int)this_00 + 0x3c) | 4;
        FUN_005f59f0(*(void **)((int)this + 0x178),(int)this_00);
        FUN_0060a280(*(void **)((int)this + 0x174));
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
      }
      iVar9 = FUN_0052d370(piVar6,*(undefined4 *)((int)this + 0x148),3);
      if (iVar9 == 0) {
        local_34 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),0x6c);
        local_30 = 0x2852;
      }
    }
    else {
      puVar7 = (undefined4 *)FUN_0060a860(*(void **)((int)this + 0x178),*puVar11 & 0xffffff);
      if (puVar7 != (undefined4 *)0x0) {
        puVar10 = FUN_0042d170(puVar7,&local_28);
        uVar1 = *puVar10;
        uVar2 = *puVar11;
        FUN_00619730();
        if (uVar1 == uVar2) {
          FUN_005f5ac0(*(void **)((int)this + 0x178),(int)puVar7);
          FUN_0060a280(*(void **)((int)this + 0x174));
          (**(code **)*puVar7)(1);
        }
      }
      local_28.left = 0x2d;
      local_28.top = 0x2e;
      local_4 = CONCAT31(local_4._1_3_,4);
      if (((*puVar11 >> 0x18 < 0x2d) || (0x2d < *puVar11 >> 0x18)) ||
         (iVar9 = FUN_0052cf40(piVar6,*(undefined4 *)((int)this + 0x148),3), iVar9 != 0)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      if (bVar3) {
        local_34 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),0x6b);
        local_30 = 0x2855;
      }
      local_28.left = 0x28;
      local_28.top = 0x29;
      local_4 = CONCAT31(local_4._1_3_,5);
      if (((*param_1 >> 0x18 < 0x28) || (0x28 < *param_1 >> 0x18)) ||
         (iVar9 = FUN_0052c8c0(piVar6,*(undefined4 *)((int)this + 0x148),3), iVar9 != 0)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      if (bVar3) {
        local_34 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),0x68);
        local_30 = 0x2858;
        if (*(int *)((int)this + 0x160) == 0x67) {
          local_28.left = 6;
          local_28.top = 0x34;
          local_28.right = 0x47;
          local_28.bottom = 0x87;
          InvalidateRect(*(HWND *)((int)this + 0x18),&local_28,0);
        }
      }
      local_28.left = 0x29;
      local_28.top = 0x2a;
      local_4 = CONCAT31(local_4._1_3_,6);
      if (((*param_1 >> 0x18 < 0x29) || (0x29 < *param_1 >> 0x18)) ||
         (iVar9 = FUN_0052c5a0(piVar6,*(undefined4 *)((int)this + 0x148),3), iVar9 != 0)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      if (bVar3) {
        local_34 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),0x69);
        local_30 = 0x285b;
        if (*(int *)((int)this + 0x160) == 0x67) {
          local_28.left = 6;
          local_28.top = 0x34;
          local_28.right = 0x47;
          local_28.bottom = 0xd8;
          InvalidateRect(*(HWND *)((int)this + 0x18),&local_28,0);
        }
      }
      local_28.left = 0x2a;
      local_28.top = 0x2b;
      local_4 = CONCAT31(local_4._1_3_,7);
      if (((*param_1 >> 0x18 < 0x2a) || (0x2a < *param_1 >> 0x18)) ||
         (iVar9 = FUN_0052c270(piVar6,*(undefined4 *)((int)this + 0x148),3), iVar9 != 0)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      puVar11 = param_1;
      if (bVar3) {
        local_34 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),0x6a);
        local_30 = 0x285e;
        puVar11 = param_1;
      }
    }
    if (*(int *)((int)this + 0x160) == 0x67) {
      local_28.left = 6;
      local_28.top = 0x34;
      local_28.right = 0x47;
      local_28.bottom = 0x128;
      InvalidateRect(*(HWND *)((int)this + 0x18),&local_28,0);
    }
    puVar7 = FUN_004f5940((void *)((int)this + 0x114),puVar11);
    FUN_004acca0((void *)((int)this + 0x114),puVar7);
    if (((local_34 != (void *)0x0) && (local_30 != 0)) &&
       (uVar4 = FUN_006034a0(local_34,0), uVar4 != local_30)) {
      FUN_00603150(local_34,0,local_30);
      InvalidateRect(*(HWND *)((int)local_34 + 0x18),(RECT *)0x0,0);
    }
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
    ExceptionList = local_c;
    return;
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

