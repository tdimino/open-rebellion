
bool __thiscall FUN_00548840(void *this,void *param_1,undefined4 param_2,int param_3,void *param_4)

{
  void *pvVar1;
  int iVar2;
  bool bVar3;
  bool bVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647ad8;
  local_c = ExceptionList;
  bVar3 = true;
  ExceptionList = &local_c;
  FUN_00402d80(param_1,&param_1);
  bVar4 = param_1 == (void *)0x71000043;
  local_4 = 0;
  FUN_00619730();
  if (bVar4) {
    if (param_3 == 0xb) {
      iVar2 = FUN_00520ac0((int)pvVar1);
      if (iVar2 < 8) goto LAB_005488da;
      iVar2 = FUN_00543af0(this,0,param_4);
    }
    else {
      if (param_3 != 8) goto LAB_005488da;
      iVar2 = FUN_00549910(this,param_4);
    }
    bVar3 = iVar2 != 0;
  }
LAB_005488da:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return bVar3;
}

