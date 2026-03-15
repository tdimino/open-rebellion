
void * __thiscall FUN_00472fd0(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632910;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar2 = (undefined4 *)FUN_00402e40(param_1,(uint *)&param_1);
  local_4 = 0;
  FUN_004ad7e0(this,puVar2);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
  *(undefined ***)this = &PTR_FUN_0065a378;
  *(undefined4 *)((int)this + 0x30) = 0;
  FUN_00473090(this,(int)pvVar1);
  ExceptionList = local_c;
  return this;
}

