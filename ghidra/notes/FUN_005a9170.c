
void __thiscall FUN_005a9170(void *this,int *param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  void *pvVar4;
  int iVar5;
  uint uVar6;
  
  FUN_005c5fb0((void *)((int)this + 0x9c4),(int)param_1);
  FUN_005c5f20((void *)((int)this + 0x988),(int)param_1);
  FUN_005a9380(this,param_1);
  FUN_005c5f20((void *)(*(int *)((int)this + param_1[0x5a] * 4 + 8) + 100),(int)param_1);
  uVar6 = param_1[0x5d];
  pvVar4 = (void *)FUN_00596bd0();
  iVar5 = FUN_005a6480(pvVar4,uVar6);
  if ((iVar5 != 0) && (*(int *)(iVar5 + 0x6c) == param_1[0x3b])) {
    iVar1 = param_1[0xd];
    iVar5 = *(int *)(iVar5 + 0x54);
    iVar2 = param_1[0xe];
    iVar3 = param_1[0xf];
    *(int *)(iVar5 + 0x3c) = param_1[0xc];
    *(int *)(iVar5 + 0x40) = iVar1;
    *(int *)(iVar5 + 0x44) = iVar2;
    *(int *)(iVar5 + 0x48) = iVar3;
  }
  iVar5 = 1;
  uVar6 = 0x1b;
  pvVar4 = (void *)FUN_005ba8b0();
  FUN_005ba980(pvVar4,uVar6,iVar5);
  param_1[0x5c] = 2;
  return;
}

