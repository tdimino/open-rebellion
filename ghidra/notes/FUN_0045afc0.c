
void __thiscall FUN_0045afc0(void *this,void *param_1)

{
  HWND hWnd;
  void *this_00;
  undefined4 *puVar1;
  uint uVar2;
  uint *puVar3;
  int iVar4;
  int *piVar5;
  undefined1 uVar6;
  RECT local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630908;
  local_c = ExceptionList;
  hWnd = *(HWND *)((int)this + 0x18);
  if (param_1 == (void *)0x0) {
    uVar6 = *(undefined1 *)((int)this + 0x158);
    ExceptionList = &local_c;
    iVar4 = FUN_005fc0f0(*(int **)((int)this + 0x19c));
    uVar2 = FUN_005fc0e0(*(int **)((int)this + 0x19c));
    FUN_005fcfe0(*(void **)((int)this + 0x19c),0,0,uVar2,iVar4,uVar6);
    for (piVar5 = (int *)(**(code **)(*(int *)((int)this + 0x174) + 8))(); piVar5 != (int *)0x0;
        piVar5 = (int *)(**(code **)(*piVar5 + 0xc))()) {
      puVar1 = (undefined4 *)FUN_004f26d0(&param_1,piVar5 + 0xf);
      local_4 = 2;
      uVar2 = FUN_005f4960(puVar1);
      local_4 = 0xffffffff;
      FUN_00619730();
      if ((uVar2 & 1) != 0) {
        local_1c.left = piVar5[0x10];
        local_1c.top = piVar5[0x11];
        local_1c.right = piVar5[0x12];
        local_1c.bottom = piVar5[0x13];
        piVar5[0xf] = uVar2 & 0xfffffffe;
        InvalidateRect(hWnd,&local_1c,1);
      }
    }
    (**(code **)(*(int *)((int)this + 0x184) + 4))();
  }
  else {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_004f26d0(&param_1,(undefined4 *)((int)param_1 + 0x3c));
    local_4 = 0;
    uVar2 = FUN_005f4960(puVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((uVar2 & 1) != 0) {
      local_1c.left = *(int *)((int)this_00 + 0x40);
      local_1c.top = *(int *)((int)this_00 + 0x44);
      local_1c.right = *(int *)((int)this_00 + 0x48);
      local_1c.bottom = *(int *)((int)this_00 + 0x4c);
      *(uint *)((int)this_00 + 0x3c) = uVar2 & 0xfffffffe;
      FUN_005fcfe0(*(void **)((int)this + 0x19c),local_1c.left,local_1c.top,
                   local_1c.right - local_1c.left,local_1c.bottom - local_1c.top,
                   *(undefined1 *)((int)this + 0x158));
      InvalidateRect(hWnd,&local_1c,1);
      puVar3 = (uint *)FUN_0042d170(this_00,&param_1);
      local_4 = 1;
      puVar1 = (undefined4 *)FUN_004f5910((void *)((int)this + 0x184),puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      if (puVar1 != (undefined4 *)0x0) {
        (**(code **)*puVar1)(1);
      }
    }
  }
  ExceptionList = local_c;
  return;
}

