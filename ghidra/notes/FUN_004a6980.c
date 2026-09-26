
void __fastcall FUN_004a6980(int param_1)

{
  int *piVar1;
  void *pvVar2;
  uint uVar3;
  int *this;
  int *this_00;
  uint uVar4;
  uint local_8;
  uint *local_4;
  
  local_8 = 0;
  uVar4 = *(uint *)(param_1 + 0x14c);
  uVar3 = *(uint *)(param_1 + 0x194);
  if ((uVar4 & 4) == 0) {
    if ((uVar4 & 0x40) == 0) {
      if ((uVar4 & 0x30) != 0) {
        *(uint *)(param_1 + 0x14c) = *(uint *)(param_1 + 0x14c) & 0xffffffcf;
        (**(code **)(**(int **)(param_1 + 0x158) + 8))();
        (**(code **)(**(int **)(param_1 + 0x15c) + 8))();
        InvalidateRect(*(HWND *)(param_1 + 0x18),(RECT *)0x0,0);
      }
      return;
    }
    if ((uVar4 & 0x20) == 0) {
      uVar4 = *(uint *)(param_1 + 0x14c);
      *(uint *)(param_1 + 0x14c) = uVar4 | 0x20;
      *(uint *)(param_1 + 0x14c) = uVar4 & 0xffffffef | 0x20;
      InvalidateRect(*(HWND *)(param_1 + 0x18),(RECT *)0x0,0);
      (**(code **)(**(int **)(param_1 + 0x158) + 8))();
    }
    this = *(int **)(param_1 + 0x15c);
    local_4 = *(uint **)(param_1 + 0x184);
  }
  else {
    if ((uVar4 & 0x10) == 0) {
      *(uint *)(param_1 + 0x14c) = uVar4 | 0x10;
      *(uint *)(param_1 + 0x14c) = uVar4 & 0xffffffdf | 0x10;
      (**(code **)(**(int **)(param_1 + 0x15c) + 8))();
      InvalidateRect(*(HWND *)(param_1 + 0x18),(RECT *)0x0,0);
    }
    this = *(int **)(param_1 + 0x158);
    local_4 = *(uint **)(param_1 + 0x180);
  }
  if ((*(byte *)(param_1 + 0x14c) & 8) == 0) {
    piVar1 = (int *)FUN_00604500(this + 0x1b,0x66);
    (**(code **)(*piVar1 + 0xc))();
    uVar4 = FUN_00603140((int)piVar1);
    FUN_006030c0(piVar1,uVar4 & 0xfffffffd);
    (**(code **)(*piVar1 + 4))(5);
  }
  else {
    if (uVar3 == 0x66) {
      uVar3 = 0x67;
    }
    piVar1 = (int *)FUN_00604500(this + 0x1b,0x66);
    FUN_006030c0(piVar1,2);
    (**(code **)(*piVar1 + 0x10))();
    (**(code **)(*piVar1 + 8))();
  }
  this_00 = this + 0x1b;
  FUN_00604500(this_00,uVar3);
  FUN_0060d7e0(this,uVar3,0);
  *(uint *)(param_1 + 0x194) = uVar3;
  for (piVar1 = (int *)(**(code **)(**(int **)(param_1 + 0x188) + 8))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 0xc))()) {
    if ((*(byte *)(piVar1 + 0xf) & 1) != 0) {
      local_8 = local_8 | piVar1[0x15];
    }
  }
  uVar4 = 0x40;
  if ((local_8 & 1) == 0) {
    pvVar2 = (void *)FUN_00604500(this_00,0x67);
    FUN_006030c0(pvVar2,uVar4);
  }
  else {
    pvVar2 = (void *)FUN_00604500(this_00,0x67);
    FUN_006030f0(pvVar2,uVar4);
  }
  uVar4 = 0x40;
  if ((local_8 & 2) == 0) {
    pvVar2 = (void *)FUN_00604500(this_00,0x68);
    FUN_006030c0(pvVar2,uVar4);
  }
  else {
    pvVar2 = (void *)FUN_00604500(this_00,0x68);
    FUN_006030f0(pvVar2,uVar4);
  }
  uVar4 = 0x40;
  if ((local_8 & 4) == 0) {
    pvVar2 = (void *)FUN_00604500(this_00,0x69);
    FUN_006030c0(pvVar2,uVar4);
  }
  else {
    pvVar2 = (void *)FUN_00604500(this_00,0x69);
    FUN_006030f0(pvVar2,uVar4);
  }
  if (local_4 != (uint *)0x0) {
    FUN_0060da10(this,local_4,this[10] + -0x61,this[0xb] - 0x1d);
  }
  (**(code **)(*this + 4))(5);
  return;
}

