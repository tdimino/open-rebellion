
int * __thiscall
FUN_004f7d50(void *this,uint *param_1,undefined4 param_2,void *param_3,int param_4,int param_5)

{
  bool bVar1;
  void *pvVar2;
  int *this_00;
  int iVar3;
  uint *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063fd98;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = FUN_0053f100(param_1);
  pvVar2 = param_3;
  if (this_00 != (int *)0x0) {
    iVar3 = (**(code **)(*this_00 + 0x94))(param_2,param_3);
    if ((iVar3 == 0) || (this_00 == (int *)0x0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_004025b0(this,(uint *)&param_1);
    local_c = (void *)0x0;
    iVar3 = (**(code **)(*this_00 + 0xa8))(puVar4,pvVar2);
    if ((iVar3 == 0) || (param_1 = (uint *)0x1, !bVar1)) {
      param_1 = (uint *)0x0;
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
    FUN_004f7480(this_00,param_4,pvVar2);
    FUN_004f74f0(this_00,param_5,pvVar2);
  }
  ExceptionList = local_c;
  return this_00;
}

