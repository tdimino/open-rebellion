
undefined4 __thiscall FUN_00536220(void *this,undefined4 param_1,uint *param_2,void *param_3)

{
  bool bVar1;
  int iVar2;
  uint *puVar3;
  int iVar4;
  
  iVar2 = FUN_00534950(this,param_3);
  puVar3 = FUN_004ece40(param_2);
  if (puVar3 == (uint *)0x0) {
    iVar4 = FUN_005344f0(this,0,param_3);
    if ((iVar4 == 0) || (iVar2 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_005345d0(this,0,param_3);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_00534640(this,0,param_3);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_005346b0(this,0,param_3);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_00534720(this,0,param_3);
    if ((iVar2 != 0) && (bVar1)) {
      return 1;
    }
  }
  else {
    iVar4 = FUN_00534560(this,0,param_3);
    if ((iVar4 != 0) && (iVar2 != 0)) {
      return 1;
    }
  }
  return 0;
}

