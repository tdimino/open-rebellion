
void * __thiscall FUN_005d8980(void *this,undefined4 param_1,undefined4 param_2)

{
  int *piVar1;
  int iVar2;
  void *unaff_EDI;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654a73;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f5590(this);
  local_4 = 0;
  FUN_005f5270((undefined4 *)((int)this + 0x1c));
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0066d320;
  *(undefined4 *)((int)this + 0x1c) = &PTR_LAB_0066d310;
  *(undefined4 *)((int)this + 0x28) = 1;
  *(undefined4 *)((int)this + 0x30) = param_2;
  FUN_005f5600(this,param_1);
  iVar2 = (**(code **)(*DAT_006bcbd0 + 0x10))
                    (DAT_006bcbd0,param_2,(undefined4 *)((int)this + 0x2c),0);
  FUN_005d8b00(iVar2);
  piVar1 = *(int **)((int)this + 0x2c);
  iVar2 = (**(code **)(*piVar1 + 0x18))(piVar1,param_1,0);
  FUN_005d8b00(iVar2);
  ExceptionList = unaff_EDI;
  return this;
}

