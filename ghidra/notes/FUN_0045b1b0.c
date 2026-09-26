
void __thiscall FUN_0045b1b0(void *this,void *param_1)

{
  HWND hWnd;
  uint *puVar1;
  undefined4 *puVar2;
  uint uVar3;
  int iVar4;
  void *pvVar5;
  void *pvVar6;
  int iVar7;
  RECT local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar6 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630952;
  local_c = ExceptionList;
  hWnd = *(HWND *)((int)this + 0x18);
  ExceptionList = &local_c;
  puVar1 = FUN_0042d170(param_1,&param_1);
  local_4 = 0;
  FUN_004f3220(*(int *)(*(int *)((int)this + 0x194) + 0x9c),puVar1);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar1 = (uint *)((int)pvVar6 + 0x3c);
  puVar2 = (undefined4 *)FUN_004f26d0(&param_1,puVar1);
  local_4 = 1;
  uVar3 = FUN_005f4960(puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((uVar3 & 2) != 0) {
    local_1c.left = *(int *)((int)pvVar6 + 0x40);
    local_1c.top = *(int *)((int)pvVar6 + 0x44);
    local_1c.right = *(LONG *)((int)pvVar6 + 0x48);
    local_1c.bottom = *(LONG *)((int)pvVar6 + 0x4c);
    *puVar1 = *puVar1 | 1;
    InvalidateRect(hWnd,&local_1c,0);
    iVar4 = *(int *)(*(int *)((int)this + 0x194) + 0x9c);
    if (iVar4 == 1) {
      iVar4 = 1;
    }
    else if (iVar4 == 2) {
      iVar4 = 2;
    }
    else {
      iVar4 = 0;
    }
    uVar3 = *puVar1 & 0x1c;
    if (uVar3 == 4) {
      iVar7 = 1;
    }
    else if (uVar3 == 8) {
      iVar7 = 2;
    }
    else {
      iVar7 = (-(uint)(uVar3 != 0x10) & 0xfffffffd) + 3;
    }
    if ((iVar4 == iVar7) && (*(int *)((int)pvVar6 + 0x54) != 0)) {
      local_1c.left = *(int *)((int)pvVar6 + 0x40);
      local_1c.top = *(int *)((int)pvVar6 + 0x44);
      local_1c.right = *(LONG *)((int)pvVar6 + 0x48);
      local_1c.bottom = *(LONG *)((int)pvVar6 + 0x4c);
      pvVar5 = *(void **)((int)pvVar6 + 0x20);
    }
    else {
      pvVar5 = (void *)0x0;
    }
    if (pvVar5 != (void *)0x0) {
      FUN_005fd0f0(pvVar5,*(uint **)((int)this + 0x19c),local_1c.left,local_1c.top);
    }
    pvVar5 = (void *)FUN_00618b70(0x20);
    local_4 = 2;
    if (pvVar5 == (void *)0x0) {
      pvVar6 = (void *)0x0;
    }
    else {
      uVar3 = *(uint *)((int)pvVar6 + 0xc);
      puVar2 = FUN_0042d170(pvVar6,&param_1);
      local_4 = CONCAT31(local_4._1_3_,3);
      pvVar6 = FUN_004f5b10(pvVar5,puVar2,uVar3 >> 0x10);
    }
    local_4 = 2;
    if (pvVar5 != (void *)0x0) {
      FUN_00619730();
    }
    local_4 = 0xffffffff;
    FUN_004f57b0((void *)((int)this + 0x184),pvVar6);
  }
  ExceptionList = local_c;
  return;
}

