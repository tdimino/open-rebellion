
undefined4 __thiscall FUN_00606bd0(void *this,int param_1,int param_2,uint param_3)

{
  bool bVar1;
  void *this_00;
  undefined4 uVar2;
  int *this_01;
  undefined3 extraout_var;
  undefined4 uVar3;
  undefined1 uVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065659b;
  local_c = ExceptionList;
  uVar3 = 0;
  ExceptionList = &local_c;
  this_00 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (this_00 == (void *)0x0) {
    this_01 = (int *)0x0;
  }
  else {
    uVar4 = 2;
    uVar2 = FUN_006037f0(*(int *)((int)this + 0x98));
    this_01 = FUN_005fbd20(this_00,uVar2,param_3,uVar4);
  }
  local_4 = 0xffffffff;
  if ((*(int *)((int)this + 0xa4) != 0) && (this_01 != (int *)0x0)) {
    bVar1 = FUN_005fc040(this_01);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      FUN_005fd0f0(this_01,*(uint **)((int)this + 0xa4),param_1,param_2);
      FUN_005fbfa0(this_01);
      FUN_00618b60((undefined *)this_01);
      uVar3 = 1;
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

