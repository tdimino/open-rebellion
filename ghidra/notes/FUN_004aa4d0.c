
void __thiscall FUN_004aa4d0(void *this,uint *param_1)

{
  bool bVar1;
  uint uVar2;
  uint local_24;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006382f0;
  local_c = ExceptionList;
  bVar1 = false;
  local_24 = 0;
  uVar2 = *param_1 >> 0x18;
  local_4 = 0;
  if ((uVar2 < 0x22) || (ExceptionList = &local_c, 0x22 < uVar2)) {
    if ((0x22 < uVar2) && (uVar2 < 0x24)) {
      bVar1 = true;
    }
    ExceptionList = &local_c;
    FUN_00619730();
    if (bVar1) goto LAB_004aa547;
    bVar1 = false;
  }
  else {
LAB_004aa547:
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    local_24 = 5;
    goto LAB_004aa69a;
  }
  uVar2 = *param_1 >> 0x18;
  bVar1 = true;
  local_4 = 1;
  if ((uVar2 < 0x24) || (0x24 < uVar2)) {
    if ((uVar2 < 0x25) || (0x25 < uVar2)) {
      bVar1 = false;
    }
    FUN_00619730();
    if (bVar1) goto LAB_004aa5bb;
    bVar1 = false;
  }
  else {
LAB_004aa5bb:
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    local_24 = 4;
  }
  else {
    if ((*param_1 >> 0x18 < 0x1c) || (0x1f < *param_1 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      local_24 = 3;
    }
    else {
      if ((*param_1 >> 0x18 < 0x10) || (0x13 < *param_1 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        local_24 = 2;
      }
      else {
        if ((*param_1 >> 0x18 < 0x30) || (0x3b < *param_1 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          local_24 = 1;
        }
      }
    }
  }
LAB_004aa69a:
  if (local_24 != *(uint *)((int)this + 0x158)) {
    FUN_0060d7e0(*(void **)((int)this + 0x160),local_24,1);
  }
  ExceptionList = local_c;
  return;
}

