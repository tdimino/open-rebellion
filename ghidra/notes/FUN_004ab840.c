
void __thiscall
FUN_004ab840(void *this,undefined4 param_1,int param_2,int param_3,int param_4,int param_5,
            int param_6,int param_7,int param_8,int param_9,undefined4 *param_10)

{
  HWND hWnd;
  int iVar1;
  int iVar2;
  void *pvVar3;
  char *pcVar4;
  HDC hDC;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063845b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined4 *)((int)this + 0xc) = param_1;
  *(undefined4 *)((int)this + 0x10) = 0;
  *(undefined4 *)((int)this + 0x14) = 0;
  *(undefined4 *)((int)this + 0x18) = 0;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x54) = 0;
  *(undefined4 *)((int)this + 0x2c) = 0x2ffffff;
  *(undefined4 *)((int)this + 0x30) = 0x2000000;
  *(undefined4 *)((int)this + 0x34) = 0x2808080;
  *(undefined4 *)((int)this + 0x3c) = 0;
  if (param_6 != 0) {
    *(int *)((int)this + 0x14) = param_6;
  }
  if (param_7 != 0) {
    *(int *)((int)this + 0x10) = param_7;
  }
  if (param_8 != 0) {
    *(int *)((int)this + 0x18) = param_8;
  }
  if (param_9 != 0) {
    *(int *)((int)this + 0x20) = param_9;
  }
  if (param_10 != (undefined4 *)0x0) {
    iVar1 = param_2;
    if (*(int **)((int)this + 0x14) != (int *)0x0) {
      *(int *)((int)this + 0x44) = param_2;
      *(int *)((int)this + 0x48) = param_3;
      iVar1 = FUN_005fc0e0(*(int **)((int)this + 0x14));
      iVar1 = iVar1 + 1 + param_2;
    }
    if (*(int **)((int)this + 0x20) != (int *)0x0) {
      *(int *)((int)this + 0x44) = iVar1;
      *(int *)((int)this + 0x48) = param_3;
      iVar2 = FUN_005fc0e0(*(int **)((int)this + 0x20));
      iVar1 = iVar1 + 1 + iVar2;
    }
    pvVar3 = (void *)FUN_00618b70(0x34);
    local_4 = 0;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_00601880(pvVar3,(char *)0x0,0,0,0,0,0,0);
    }
    local_4 = 0xffffffff;
    *(void **)((int)this + 0x38) = pvVar3;
    pcVar4 = (char *)FUN_00583c40((int)param_10);
    FUN_00601aa0(*(void **)((int)this + 0x38),pcVar4);
    FUN_00601b30(*(void **)((int)this + 0x38),iVar1,param_3);
    (**(code **)*param_10)(1);
  }
  if (*(int *)((int)this + 0x40) == 0) {
    *(int *)((int)this + 0x60) = param_2;
    *(int *)((int)this + 100) = param_3;
    *(int *)((int)this + 0x68) = param_4 + param_2;
    *(int *)((int)this + 0x6c) = param_5 + param_3;
  }
  else {
    hWnd = *(HWND *)(*(int *)((int)this + 0x40) + 0x18);
    hDC = GetDC(hWnd);
    FUN_00601b80(*(void **)((int)this + 0x38),hDC);
    ReleaseDC(hWnd,hDC);
    iVar1 = *(int *)((int)this + 0x38);
    *(undefined4 *)((int)this + 0x60) = *(undefined4 *)(iVar1 + 0x14);
    *(undefined4 *)((int)this + 100) = *(undefined4 *)(iVar1 + 0x18);
    *(undefined4 *)((int)this + 0x68) = *(undefined4 *)(iVar1 + 0x1c);
    *(undefined4 *)((int)this + 0x6c) = *(undefined4 *)(iVar1 + 0x20);
    *(undefined4 *)((int)this + 0x60) = 1;
    *(undefined4 *)((int)this + 0x68) = *(undefined4 *)(*(int *)((int)this + 0x40) + 0x30);
  }
  if ((*(int **)((int)this + 0x14) != (int *)0x0) &&
     (iVar1 = FUN_005fc0f0(*(int **)((int)this + 0x14)),
     *(short *)((int)this + 0x6c) < (short)((short)iVar1 + *(short *)((int)this + 100)))) {
    iVar1 = FUN_005fc0f0(*(int **)((int)this + 0x14));
    *(int *)((int)this + 0x6c) = iVar1 + *(int *)((int)this + 100);
  }
  if ((*(int **)((int)this + 0x20) != (int *)0x0) &&
     (iVar1 = FUN_005fc0f0(*(int **)((int)this + 0x20)),
     *(short *)((int)this + 0x6c) < (short)((short)iVar1 + *(short *)((int)this + 100)))) {
    iVar1 = FUN_005fc0f0(*(int **)((int)this + 0x20));
    *(int *)((int)this + 0x6c) = iVar1 + *(int *)((int)this + 100);
  }
  ExceptionList = pvStack_c;
  return;
}

