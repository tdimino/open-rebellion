
void __thiscall FUN_005f4fa0(void *this,int param_1)

{
  void *pvVar1;
  void *pvVar2;
  int iVar3;
  void *pvVar4;
  void *pvVar5;
  
  pvVar5 = *(void **)(param_1 + 4);
  pvVar1 = *(void **)(param_1 + 8);
  iVar3 = FUN_005f5c60(param_1);
  if (iVar3 != 0) {
    *(undefined4 *)(iVar3 + 0x10) = *(undefined4 *)(param_1 + 0x10);
  }
  if (pvVar5 == (void *)0x0) {
    pvVar4 = *(void **)(param_1 + 0xc);
    if (pvVar1 == (void *)0x0) {
      if (pvVar4 != (void *)0x0) {
        FUN_005ee410(pvVar4,param_1,0);
      }
      pvVar5 = (void *)0x0;
    }
    else {
      *(void **)((int)pvVar1 + 0xc) = pvVar4;
      pvVar5 = pvVar1;
      if (pvVar4 != (void *)0x0) {
        FUN_005ee410(pvVar4,param_1,pvVar1);
      }
    }
  }
  else if (pvVar1 == (void *)0x0) {
    pvVar4 = *(void **)(param_1 + 0xc);
    *(void **)((int)pvVar5 + 0xc) = pvVar4;
    if (pvVar4 != (void *)0x0) {
      FUN_005ee410(pvVar4,param_1,pvVar5);
    }
  }
  else {
    pvVar4 = *(void **)(param_1 + 0xc);
    *(void **)((int)pvVar1 + 0xc) = pvVar4;
    if (pvVar4 != (void *)0x0) {
      FUN_005ee410(pvVar4,param_1,pvVar1);
    }
    pvVar4 = pvVar1;
    for (pvVar2 = *(void **)((int)pvVar1 + 4); pvVar2 != (void *)0x0;
        pvVar2 = *(void **)((int)pvVar2 + 4)) {
      pvVar4 = pvVar2;
    }
    *(void **)((int)pvVar5 + 0xc) = pvVar4;
    *(void **)((int)pvVar4 + 4) = pvVar5;
    pvVar5 = pvVar1;
  }
  if (*(int *)((int)this + 4) == param_1) {
    *(void **)((int)this + 4) = pvVar5;
  }
  FUN_005ee3f0(param_1);
  FUN_005f5100(this,(int)pvVar4);
  return;
}

