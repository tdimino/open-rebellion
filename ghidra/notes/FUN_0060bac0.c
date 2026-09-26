
void * __thiscall FUN_0060bac0(void *this,int param_1)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  uint *puVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656a24;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f5270(this);
  local_4 = 0;
  FUN_005f2f50((undefined4 *)((int)this + 0x14));
  local_4._0_1_ = 1;
  FUN_005f4950((undefined4 *)((int)this + 0x3c),0);
  *(undefined ***)this = &PTR_FUN_0066e1b0;
  local_4._0_1_ = 2;
  *(undefined4 *)((int)this + 0xc) = *(undefined4 *)(param_1 + 0xc);
  *(undefined2 *)((int)this + 0x10) = *(undefined2 *)(param_1 + 0x10);
  *(undefined2 *)((int)this + 0x12) = *(undefined2 *)(param_1 + 0x12);
  *(undefined4 *)((int)this + 0x28) = *(undefined4 *)(param_1 + 0x28);
  *(undefined4 *)((int)this + 0x2c) = *(undefined4 *)(param_1 + 0x2c);
  *(undefined4 *)((int)this + 0x30) = *(undefined4 *)(param_1 + 0x30);
  *(undefined4 *)((int)this + 0x34) = *(undefined4 *)(param_1 + 0x34);
  *(undefined4 *)((int)this + 0x38) = *(undefined4 *)(param_1 + 0x38);
  *(undefined4 *)((int)this + 0x50) = *(undefined4 *)(param_1 + 0x50);
  *(undefined4 *)((int)this + 0x3c) = *(undefined4 *)(param_1 + 0x3c);
  *(undefined4 *)((int)this + 0x54) = *(undefined4 *)(param_1 + 0x54);
  *(undefined4 *)((int)this + 0x58) = *(undefined4 *)(param_1 + 0x58);
  *(undefined4 *)((int)this + 0x5c) = *(undefined4 *)(param_1 + 0x5c);
  *(undefined4 *)((int)this + 0x60) = *(undefined4 *)(param_1 + 0x60);
  *(undefined4 *)((int)this + 100) = *(undefined4 *)(param_1 + 100);
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  CopyRect((LPRECT)((int)this + 0x40),(RECT *)(param_1 + 0x40));
  FUN_005f3090((undefined4 *)((int)this + 0x14),param_1 + 0x14);
  if (*(int *)(param_1 + 0x20) != 0) {
    pvVar1 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 3;
    if (pvVar1 == (void *)0x0) {
      puVar4 = (uint *)0x0;
    }
    else {
      puVar4 = *(uint **)(param_1 + 0x20);
      iVar2 = FUN_005fc0f0((int *)puVar4);
      iVar3 = FUN_005fc0e0(*(int **)(param_1 + 0x20));
      puVar4 = FUN_005fbda0(pvVar1,iVar3,iVar2,puVar4);
    }
    *(uint **)((int)this + 0x20) = puVar4;
    local_4._0_1_ = 2;
    FUN_005fcc30(*(void **)(param_1 + 0x20),puVar4,0,0,0,0,(undefined4 *)0x0,0);
  }
  if (*(int *)(param_1 + 0x24) != 0) {
    pvVar1 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 4;
    if (pvVar1 == (void *)0x0) {
      puVar4 = (uint *)0x0;
    }
    else {
      puVar4 = *(uint **)(param_1 + 0x24);
      iVar2 = FUN_005fc0f0((int *)puVar4);
      iVar3 = FUN_005fc0e0(*(int **)(param_1 + 0x24));
      puVar4 = FUN_005fbda0(pvVar1,iVar3,iVar2,puVar4);
    }
    *(uint **)((int)this + 0x24) = puVar4;
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_005fcc30(*(void **)(param_1 + 0x24),puVar4,0,0,0,0,(undefined4 *)0x0,0);
  }
  ExceptionList = pvStack_c;
  return this;
}

