
void __thiscall FUN_004a73c0(void *this,uint *param_1)

{
  int iVar1;
  bool bVar2;
  int *piVar3;
  int *piVar4;
  uint *puVar5;
  uint uVar6;
  void *this_00;
  int *this_01;
  int *piVar7;
  uint uVar8;
  int *local_1c;
  int local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637f18;
  local_c = ExceptionList;
  piVar7 = (int *)0x0;
  local_1c = (int *)0x0;
  ExceptionList = &local_c;
  piVar4 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x148) + 0x9c),param_1);
  local_18[1] = 0x14;
  local_18[2] = 0x1c;
  if ((*param_1 >> 0x18 < 0x14) || (0x1b < *param_1 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    this_01 = local_1c;
    piVar7 = piVar4;
    piVar3 = local_1c;
    if (piVar4 != (int *)0x0) {
      this_01 = (int *)piVar4[7];
    }
  }
  else {
    local_18[1] = 8;
    local_18[2] = 0x10;
    if ((*param_1 >> 0x18 < 8) || (0xf < *param_1 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    this_01 = piVar4;
    piVar3 = piVar4;
    if (((!bVar2) && (this_01 = local_1c, piVar3 = local_1c, piVar4 != (int *)0x0)) &&
       (piVar7 = (int *)piVar4[7], piVar7 != (int *)0x0)) {
      this_01 = (int *)piVar7[7];
      piVar3 = (int *)piVar7[7];
    }
  }
  local_1c = piVar3;
  local_18[0] = 0;
  if (this_01 != (int *)0x0) {
    *(uint *)((int)this + 0x14c) = *(uint *)((int)this + 0x14c) & 0xfffffffe;
    puVar5 = FUN_004025b0(this_01,(uint *)&local_1c);
    local_4 = 0;
    local_18[0] = FUN_0060a860(*(void **)((int)this + 0x188),*puVar5 & 0xffffff);
    local_4 = 0xffffffff;
    FUN_00619730();
    *(uint *)(local_18[0] + 0x3c) = *(uint *)(local_18[0] + 0x3c) | 1;
    FUN_0060a280(*(void **)((int)this + 0x160));
  }
  if (local_18[0] != 0) {
    if ((piVar7 == (int *)0x0) || (piVar7 != piVar4)) {
      if (local_18[0] == 0) {
        ExceptionList = local_c;
        return;
      }
      if (piVar7 == (int *)0x0) {
        ExceptionList = local_c;
        return;
      }
      if (piVar7 == piVar4) {
        ExceptionList = local_c;
        return;
      }
      *(uint *)((int)this + 0x14c) = *(uint *)((int)this + 0x14c) & 0xfffffffe;
      if ((~*(uint *)(local_18[0] + 0x68) & 2) == 0) {
        uVar8 = *(uint *)(local_18[0] + 0x68) & 0xfffffffd;
      }
      else {
        uVar8 = *(uint *)(local_18[0] + 0x68) | 2;
      }
      *(uint *)(local_18[0] + 0x68) = uVar8;
      FUN_004a3d40(this,(int)this_01,local_18);
      FUN_0060a280(*(void **)((int)this + 0x160));
      bVar2 = true;
      *(uint *)((int)this + 0x14c) = *(uint *)((int)this + 0x14c) | 1;
      iVar1 = *(int *)(*(int *)((int)this + 0x148) + 0x9c);
      if (iVar1 == 1) {
        uVar8 = 1;
      }
      else if (iVar1 == 2) {
        uVar8 = 2;
      }
      else {
        uVar8 = 0;
      }
      if (((uint)this_01[9] >> 6 & 3) == uVar8) {
        this_00 = *(void **)((int)this + 0x158);
      }
      else {
        this_00 = *(void **)((int)this + 0x15c);
      }
      local_18[1] = 0x1c;
      local_18[2] = 0x20;
      if ((*param_1 >> 0x18 < 0x1c) || (0x1f < *param_1 >> 0x18)) {
        bVar2 = false;
      }
      FUN_00619730();
      if (bVar2) {
        uVar8 = 0x67;
      }
      else {
        local_18[1] = 0x10;
        local_18[2] = 0x14;
        if ((*param_1 >> 0x18 < 0x10) || (0x13 < *param_1 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (bVar2) {
          uVar8 = 0x68;
        }
        else {
          local_18[1] = 0x30;
          local_18[2] = 0x40;
          if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          if (!bVar2) {
            ExceptionList = local_c;
            return;
          }
          uVar8 = 0x69;
        }
      }
    }
    else {
      *(uint *)((int)this + 0x14c) = *(uint *)((int)this + 0x14c) | 1;
      iVar1 = *(int *)(*(int *)((int)this + 0x148) + 0x9c);
      if (iVar1 == 1) {
        uVar6 = 1;
      }
      else if (iVar1 == 2) {
        uVar6 = 2;
      }
      else {
        uVar6 = 0;
      }
      uVar8 = 0x66;
      if (((uint)this_01[9] >> 6 & 3) == uVar6) {
        this_00 = *(void **)((int)this + 0x158);
      }
      else {
        this_00 = *(void **)((int)this + 0x15c);
      }
    }
    FUN_0060d7e0(this_00,uVar8,1);
  }
  ExceptionList = local_c;
  return;
}

