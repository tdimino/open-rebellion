
void * __thiscall
FUN_00441a60(void *this,int param_1,uint *param_2,int param_3,int param_4,int param_5,int param_6,
            undefined4 param_7)

{
  HDC hDC;
  void *pvVar1;
  int iVar2;
  uint uVar3;
  DWORD DVar4;
  undefined1 uVar5;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ea9c;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004aa8c0((undefined4 *)((int)this + 0x18));
  local_4 = 0;
  FUN_004aa7f0((undefined4 *)((int)this + 0x24));
  *(undefined ***)this = &PTR_LAB_00659b60;
  *(int *)((int)this + 0x10) = param_1;
  *(undefined4 *)((int)this + 0x14) = *(undefined4 *)(param_1 + 0x18);
  local_4._0_1_ = 1;
  *(int *)((int)this + 0x50) = param_3;
  *(int *)((int)this + 0x54) = param_4;
  if (param_5 < 1) {
    param_5 = *(int *)(param_1 + 0x38) - param_3;
  }
  if (param_6 < 1) {
    param_6 = *(int *)(param_1 + 0x3c) - param_4;
  }
  *(int *)((int)this + 0x58) = param_3;
  *(int *)((int)this + 0x5c) = param_4;
  *(int *)((int)this + 0x60) = param_5 + param_3;
  *(int *)((int)this + 100) = param_6 + param_4;
  hDC = GetDC(*(HWND *)((int)this + 0x14));
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 2;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbea0(pvVar1,param_5,param_6,param_2,hDC);
  }
  local_4._0_1_ = 1;
  *(void **)((int)this + 4) = pvVar1;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 3;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005fbea0(pvVar1,param_5,param_6,param_2,hDC);
  }
  uVar5 = 0;
  local_4 = CONCAT31(local_4._1_3_,1);
  *(void **)((int)this + 8) = pvVar1;
  iVar2 = FUN_005fc0f0(*(int **)((int)this + 4));
  uVar3 = FUN_005fc0e0(*(int **)((int)this + 4));
  FUN_005fcfe0(*(void **)((int)this + 4),0,0,uVar3,iVar2,uVar5);
  uVar5 = 0;
  iVar2 = FUN_005fc0f0(*(int **)((int)this + 8));
  uVar3 = FUN_005fc0e0(*(int **)((int)this + 8));
  FUN_005fcfe0(*(void **)((int)this + 8),0,0,uVar3,iVar2,uVar5);
  *(undefined4 *)((int)this + 0xc) = 0;
  ReleaseDC(*(HWND *)((int)this + 0x14),hDC);
  *(undefined4 *)((int)this + 0x30) = 0xfffffffe;
  *(undefined4 *)((int)this + 0x34) = param_7;
  DVar4 = timeGetTime();
  *(DWORD *)((int)this + 0x38) = DVar4;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x40) = 1;
  *(undefined4 *)((int)this + 0x68) = 1;
  *(undefined4 *)((int)this + 0x48) = 0;
  *(undefined4 *)((int)this + 0x4c) = 0;
  ExceptionList = pvStack_c;
  return this;
}

