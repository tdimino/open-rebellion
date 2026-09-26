
void __thiscall
FUN_004ab3a0(void *this,undefined4 param_1,int param_2,int param_3,uint param_4,uint param_5)

{
  int nHeight;
  int iVar1;
  int X;
  void *pvVar2;
  int iVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006383d6;
  local_c = ExceptionList;
  pvVar2 = *(void **)((int)this + 0x140);
  do {
    ExceptionList = &local_c;
    if (pvVar2 == (void *)0x0) {
LAB_004ab4a0:
      if (*(int *)((int)this + 0x11c) == 0) {
        pvVar2 = (void *)FUN_00618b70(0x28);
        local_4 = 0;
        if (pvVar2 == (void *)0x0) {
          pvVar2 = (void *)0x0;
        }
        else {
          pvVar2 = FUN_005fbd20(pvVar2,param_1,param_4,10);
        }
        local_4 = 0xffffffff;
        *(void **)((int)this + 0x11c) = pvVar2;
        pvVar2 = (void *)FUN_00618b70(0x28);
        local_4 = 1;
        if (pvVar2 == (void *)0x0) {
          pvVar2 = (void *)0x0;
        }
        else {
          pvVar2 = FUN_005fbd20(pvVar2,param_1,param_5,10);
        }
        *(void **)((int)this + 0x120) = pvVar2;
      }
      ExceptionList = local_c;
      return;
    }
    if (*(int *)((int)pvVar2 + 0xc) == param_2) {
      ExceptionList = &local_c;
      FUN_004aba40(pvVar2,param_3);
      if ((*(int *)((int)this + 0xbc) == 0) || (*(int *)((int)this + 0xc0) == 0)) {
        iVar1 = *(int *)((int)this + 0x2c) + *(int *)((int)this + 0x3c);
      }
      else {
        iVar1 = *(int *)((int)this + 200);
      }
      *(void **)(param_3 + 0x148) = this;
      X = (*(int *)((int)this + 0x28) - *(int *)(param_3 + 0x30)) + 3;
      if (X < 0) {
        X = *(int *)((int)this + 0x30) + -3 + *(int *)((int)this + 0x28);
      }
      nHeight = *(int *)(param_3 + 0x3c);
      iVar3 = *(int *)((int)this + 0x2c);
      if (*(int *)((int)pvVar2 + 100) + nHeight + iVar3 < iVar1) {
        iVar3 = *(int *)((int)pvVar2 + 100) + -3 + iVar3;
      }
      else {
        iVar3 = ((iVar3 + *(int *)((int)pvVar2 + 0x6c)) - nHeight) + 3;
      }
      MoveWindow(*(HWND *)(param_3 + 0x18),X,iVar3,*(int *)(param_3 + 0x38),nHeight,0);
      goto LAB_004ab4a0;
    }
    pvVar2 = *(void **)((int)pvVar2 + 8);
  } while( true );
}

