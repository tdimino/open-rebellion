
int __thiscall FUN_00435db0(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  undefined4 auStack_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  pvVar1 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062d508;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f4d90(param_1,(int)this + 0x1c);
  FUN_005f4d90(pvVar1,(int)this + 0x20);
  FUN_005f4d90(pvVar1,this);
  FUN_005f4d90(pvVar1,(int)this + 4);
  FUN_005f4d90(pvVar1,(int)this + 0x18);
  FUN_005f4d90(pvVar1,(int)this + 8);
  (**(code **)(*(int *)((int)this + 0x24) + 4))();
  FUN_005f4d90(pvVar1,&param_1);
  iVar2 = FUN_00436690(this,(int)param_1,0);
  iVar3 = 0;
  if (iVar2 != 0) {
    (**(code **)(**(int **)((int)this + 0x10) + 4))(pvVar1);
    FUN_005f4d90(pvVar1,&param_1);
    iVar3 = FUN_00436690(this,(int)param_1,1);
    if (iVar3 != 0) {
      (**(code **)(**(int **)((int)this + 0x14) + 4))(pvVar1);
    }
  }
  FUN_005f5650(auStack_14);
  uStack_4 = 0;
  FUN_005f57f0(auStack_14,pvVar1);
  uStack_4 = 0xffffffff;
  FUN_005f56a0(auStack_14);
  ExceptionList = pvStack_c;
  return iVar3;
}

