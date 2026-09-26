
uint __thiscall FUN_00607610(void *this,int *param_1,undefined4 param_2)

{
  bool bVar1;
  undefined4 *puVar2;
  int *this_00;
  undefined3 extraout_var;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065661b;
  local_c = ExceptionList;
  uVar3 = 0;
  ExceptionList = &local_c;
  puVar2 = (undefined4 *)FUN_00618b70(0x28);
  local_4 = 0;
  if (puVar2 == (undefined4 *)0x0) {
    this_00 = (int *)0x0;
  }
  else {
    this_00 = FUN_0060f4f0(puVar2);
  }
  local_4 = 0xffffffff;
  if ((param_1 != (int *)0x0) && (this_00 != (int *)0x0)) {
    bVar1 = FUN_005fc040(param_1);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      FUN_005f5600(this_00,param_2);
      (**(code **)(*this_00 + 8))(param_1);
      uVar3 = FUN_005f5440((void *)((int)this + 0xf8),this_00);
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

