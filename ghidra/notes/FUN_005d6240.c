
int * __thiscall FUN_005d6240(void *this,int param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int iVar3;
  undefined4 *puVar4;
  int *local_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00654923;
  pvStack_c = ExceptionList;
  local_14 = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_10 = this;
  iVar2 = FUN_005c1070();
  if (iVar2 != 0) {
    for (iVar2 = FUN_005f5060((int)this + 0x10); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
      bVar1 = FUN_005f33f0((void *)(iVar2 + 0x18),param_1);
      if (CONCAT31(extraout_var,bVar1) != 0) goto LAB_005d629d;
    }
    iVar2 = 0;
LAB_005d629d:
    if (iVar2 == 0) {
      iVar2 = (**(code **)(*(int *)((int)this + 0xc) + 4))(param_1);
    }
    else {
      iVar2 = *(int *)(iVar2 + 0x24);
    }
    if (iVar2 != 0) {
      iVar3 = FUN_005c1070();
      (**(code **)(**(int **)(iVar3 + 0x124) + 0x28))(*(int **)(iVar3 + 0x124),iVar2,&local_14);
    }
  }
  if (local_14 != (int *)0x0) {
    (**(code **)(*local_14 + 4))(local_14);
    puVar4 = (undefined4 *)FUN_00618b70(0x28);
    uStack_4 = 0;
    if (puVar4 == (undefined4 *)0x0) {
      puVar4 = (undefined4 *)0x0;
    }
    else {
      FUN_005f5c10(puVar4);
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      FUN_005f2f90(puVar4 + 6,param_1);
      *puVar4 = &PTR_FUN_0066d2dc;
      puVar4[9] = local_14;
    }
    uStack_4 = 0xffffffff;
    FUN_005f4f10((void *)((int)local_10 + 4),(int)puVar4);
  }
  ExceptionList = pvStack_c;
  return local_14;
}

