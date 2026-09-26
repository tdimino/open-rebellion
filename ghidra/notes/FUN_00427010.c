
void __thiscall FUN_00427010(void *this,uint *param_1)

{
  undefined4 uVar1;
  undefined4 *this_00;
  void *pvVar2;
  int iVar3;
  int iVar4;
  uint *puVar5;
  int iVar6;
  uint uVar7;
  undefined4 *puVar8;
  uint uVar9;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c137;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(7);
  if (param_1 == (uint *)0x0) {
    param_1 = (uint *)FUN_00618b70(0x28);
    local_4 = 1;
    if (param_1 == (uint *)0x0) {
      this_00 = (undefined4 *)0x0;
    }
    else {
      this_00 = FUN_005fbd20(param_1,uVar1,0x386,2);
    }
  }
  else {
    param_1 = (uint *)FUN_00618b70(0x28);
    local_4 = 0;
    if (param_1 == (uint *)0x0) {
      this_00 = (undefined4 *)0x0;
    }
    else {
      this_00 = FUN_005fbd20(param_1,uVar1,0x387,2);
    }
  }
  local_4 = 0xffffffff;
  if (*(int *)((int)this + 0x9c) == 1) {
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4 = 2;
    if (pvVar2 != (void *)0x0) {
      param_1 = FUN_005fbd20(pvVar2,uVar1,900,2);
      goto LAB_00427134;
    }
  }
  else {
    if (*(int *)((int)this + 0x9c) != 2) goto LAB_00427134;
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4 = 3;
    if (pvVar2 != (void *)0x0) {
      param_1 = FUN_005fbd20(pvVar2,uVar1,0x385,2);
      goto LAB_00427134;
    }
  }
  param_1 = (uint *)0x0;
LAB_00427134:
  local_4 = 0xffffffff;
  pvVar2 = (void *)FUN_00618b70(0x28);
  local_4 = 4;
  if (pvVar2 == (void *)0x0) {
    puVar5 = (uint *)0x0;
  }
  else {
    puVar5 = param_1;
    iVar3 = FUN_005fc0f0((int *)param_1);
    iVar4 = FUN_005fc0e0((int *)param_1);
    puVar5 = FUN_005fbda0(pvVar2,iVar4,iVar3,puVar5);
  }
  local_4 = 0xffffffff;
  FUN_005ff2d0(this,puVar5,0,0);
  uVar9 = 0;
  puVar8 = (undefined4 *)0x0;
  uVar7 = 0;
  iVar6 = 0;
  iVar4 = (-(uint)(*(int *)((int)this + 0x9c) != 1) & 2) + 0x19;
  iVar3 = (-(uint)(*(int *)((int)this + 0x9c) != 1) & 0x3f) + 0x15;
  puVar5 = (uint *)FUN_005ff440((int)this);
  FUN_005fcc30(this_00,puVar5,iVar3,iVar4,iVar6,uVar7,puVar8,uVar9);
  iVar4 = 0;
  iVar3 = 0;
  puVar5 = (uint *)FUN_005ff440((int)this);
  FUN_005fd0f0(param_1,puVar5,iVar3,iVar4);
  FUN_0042d230(this,0,(int)*(short *)((int)this + 0x114),1);
  FUN_00601dc0((void *)((int)this + 0x178),*(HWND *)((int)this + 0x18),1);
  FUN_00601dc0((void *)((int)this + 0x1ac),*(HWND *)((int)this + 0x18),1);
  FUN_00601dc0((void *)((int)this + 0x1e0),*(HWND *)((int)this + 0x18),1);
  InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
  if (this_00 != (undefined4 *)0x0) {
    FUN_005fbfa0(this_00);
    FUN_00618b60((undefined *)this_00);
  }
  if (param_1 != (uint *)0x0) {
    FUN_005fbfa0(param_1);
    FUN_00618b60((undefined *)param_1);
  }
  FUN_005ff440((int)this);
  ExceptionList = pvStack_c;
  return;
}

