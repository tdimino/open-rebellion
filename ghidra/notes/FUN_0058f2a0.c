
int __thiscall FUN_0058f2a0(void *this,int *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int *this_00;
  int *piVar2;
  uint uVar3;
  uint *puVar4;
  int local_7c;
  int local_78;
  uint local_74;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650378;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  local_7c = 0;
  bVar1 = FUN_0058fa30(this,param_1,&local_7c);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    piVar2 = &local_78;
    this_00 = FUN_00584e60(param_1,local_70,&local_7c);
    FUN_00617760(this_00,piVar2);
    puVar4 = &local_74;
    piVar2 = FUN_00584e60(param_1,local_70,&local_7c);
    FUN_00617760(piVar2,(int *)puVar4);
    uVar3 = *(uint *)((int)this + 0x24) & 0xffffff | local_78 << 0x18;
    *(uint *)((int)this + 0x24) = uVar3;
    *(uint *)((int)this + 0x24) = (local_74 ^ uVar3) & 0xffffff ^ uVar3;
  }
  FUN_00584fd0(param_1,&local_7c);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return CONCAT31(extraout_var,bVar1);
}

