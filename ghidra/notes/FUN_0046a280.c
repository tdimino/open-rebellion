
int * __thiscall FUN_0046a280(void *this,uint param_1)

{
  bool bVar1;
  void *this_00;
  int *piVar2;
  undefined3 extraout_var;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00631c7b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (this_00 == (void *)0x0) {
    piVar2 = (int *)0x0;
  }
  else {
    piVar2 = FUN_005fbd20(this_00,*(undefined4 *)((int)this + 0x120),param_1,2);
  }
  local_4 = 0xffffffff;
  if (piVar2 != (int *)0x0) {
    bVar1 = FUN_005fc040(piVar2);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      FUN_005fbfa0(piVar2);
      FUN_00618b60((undefined *)piVar2);
      piVar2 = FUN_0045f970((short)param_1,*(undefined4 *)((int)this + 0x124));
    }
  }
  ExceptionList = local_c;
  return piVar2;
}

