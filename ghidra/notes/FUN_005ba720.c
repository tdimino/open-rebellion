
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005ba720(void *this,undefined4 param_1)

{
  undefined4 uVar1;
  int *piVar2;
  void *pvVar3;
  int iVar4;
  void *pvStack_24;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065356b;
  pvStack_c = ExceptionList;
  uVar1 = *(undefined4 *)((int)this + 0x18);
  ExceptionList = &pvStack_c;
  *(undefined4 *)((int)this + 0x6c) = 0;
  piVar2 = (int *)FUN_005aaf90();
  pvStack_24 = (void *)0x1;
  (**(code **)(*piVar2 + 0x28))(0x44,param_1,uVar1);
  pvVar3 = (void *)FUN_00618b70(0x220);
  pvStack_c = pvVar3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    uVar1 = *(undefined4 *)((int)this + 0x18);
    iVar4 = FUN_005aaf90();
    pvStack_24 = *(void **)(iVar4 + 0x87c);
    pvVar3 = FUN_005ce500(pvVar3,(float)pvStack_24 + _DAT_0066c70c,3,param_1,uVar1,(uint *)0x0);
  }
  piVar2 = (int *)FUN_005aaf90();
  (**(code **)(*piVar2 + 0x2c))(pvVar3,1);
  ExceptionList = pvStack_24;
  return;
}

