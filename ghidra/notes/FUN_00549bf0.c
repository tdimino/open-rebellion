
/* WARNING: Type propagation algorithm not settling */

bool __thiscall
FUN_00549bf0(void *this,int param_1,int param_2,int param_3,int param_4,uint *param_5,int *param_6,
            int param_7,undefined4 param_8,uint param_9)

{
  int iVar1;
  uint *puVar2;
  bool bVar3;
  uint local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647ce0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(local_18);
  local_18[1] = 1;
  local_18[2] = 0xffffffff;
  local_4 = 1;
  iVar1 = FUN_0054bf00(param_7,local_18,local_18 + 1);
  bVar3 = iVar1 != 0;
  puVar2 = FUN_004ece40(local_18);
  if (puVar2 != (uint *)0x0) {
    iVar1 = FUN_00547ba0(this,param_2,param_3,param_4,(uint *)(param_1 + 8),param_5,param_6,local_18
                         ,param_7,param_8,(void *)0x0,0,0,param_9);
    if ((iVar1 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return bVar3;
}

