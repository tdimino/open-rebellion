
int __thiscall FUN_005d5ed0(void *this,short *param_1)

{
  int iVar1;
  int iVar2;
  undefined4 *puVar3;
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065489b;
  pvStack_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_005c1070();
  if (iVar1 != 0) {
    iVar1 = FUN_005f5060((int)this + 0x10);
    if (iVar1 != 0) {
      do {
        if (*(short *)(iVar1 + 0x18) == *param_1) goto LAB_005d5f21;
        iVar1 = *(int *)(iVar1 + 0x10);
      } while (iVar1 != 0);
    }
    iVar1 = 0;
LAB_005d5f21:
    if (iVar1 == 0) {
      iVar1 = (**(code **)(*(int *)((int)this + 0xc) + 4))(param_1);
    }
    else {
      iVar1 = *(int *)(iVar1 + 0x1c);
    }
    if (iVar1 != 0) {
      iVar2 = FUN_005c1070();
      (**(code **)(**(int **)(iVar2 + 0x128) + 0x28))(*(int **)(iVar2 + 0x128),iVar1,&local_10);
    }
  }
  if (local_10 != 0) {
    puVar3 = (undefined4 *)FUN_00618b70(0x20);
    uStack_4 = 0;
    if (puVar3 == (undefined4 *)0x0) {
      puVar3 = (undefined4 *)0x0;
    }
    else {
      FUN_005f5c10(puVar3);
      *(short *)(puVar3 + 6) = *param_1;
      puVar3[7] = local_10;
      *puVar3 = &PTR_FUN_0066d2cc;
    }
    uStack_4 = 0xffffffff;
    FUN_005f4f10((void *)((int)this + 4),(int)puVar3);
  }
  ExceptionList = pvStack_c;
  return local_10;
}

