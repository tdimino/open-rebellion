
void __fastcall FUN_004a0ca0(int *param_1)

{
  uint uVar1;
  void *pvVar2;
  uint uVar3;
  uint uVar4;
  uint local_8;
  uint local_4;
  
  if (*(int *)(param_1[0x53] + 0x9c) == 1) {
    uVar4 = 1;
  }
  else if (*(int *)(param_1[0x53] + 0x9c) == 2) {
    uVar4 = 2;
  }
  else {
    uVar4 = 0;
  }
  uVar3 = 0;
  if (param_1[0x71] != 0) {
    uVar1 = FUN_004a24b0(param_1);
    if (uVar1 != 0) {
      uVar4 = uVar1;
    }
    uVar1 = 0;
    local_8 = 0;
    local_4 = 0;
    if (uVar4 == 1) {
      uVar3 = 0x2d28;
      uVar1 = 0x2d29;
      local_8 = 0x2d2a;
      local_4 = 0x2d2b;
    }
    else if (uVar4 == 2) {
      uVar3 = 0x2d2d;
      uVar1 = 0x2d2e;
      local_8 = 0x2d2f;
      local_4 = 0x2d30;
    }
    pvVar2 = (void *)FUN_00604500((void *)(param_1[0x57] + 0x6c),0x17);
    FUN_00603150(pvVar2,0,uVar3);
    FUN_00603150(pvVar2,1,uVar1);
    pvVar2 = (void *)FUN_00604500((void *)(param_1[0x57] + 0x6c),0x18);
    FUN_00603150(pvVar2,0,local_8);
    FUN_00603150(pvVar2,1,local_4);
    FUN_0060d7b0(param_1[0x57]);
    (**(code **)(*(int *)param_1[0x70] + 4))(5);
    (**(code **)(*(int *)param_1[0x57] + 4))(5);
    InvalidateRect(*(HWND *)(param_1[0x57] + 0x18),(RECT *)0x0,0);
    FUN_004a1ba0(param_1,0x17,1);
    FUN_004a10a0(param_1);
    return;
  }
  (**(code **)(*(int *)param_1[0x70] + 8))();
  (**(code **)(*(int *)param_1[0x57] + 8))();
  FUN_004a10a0(param_1);
  return;
}

