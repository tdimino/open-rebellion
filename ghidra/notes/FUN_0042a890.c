
void __thiscall FUN_0042a890(void *this,uint *param_1,undefined4 param_2)

{
  uint uVar1;
  uint *puVar2;
  int *piVar3;
  uint *puVar4;
  void *this_00;
  int iVar5;
  undefined4 uVar6;
  int iVar7;
  
  puVar2 = param_1;
  piVar3 = FUN_004f2f60(*(int *)((int)this + 0x9c),param_1);
  puVar4 = FUN_004025b0((void *)piVar3[7],(uint *)&param_1);
  uVar1 = *puVar4;
  FUN_00619730();
  this_00 = (void *)FUN_00604500((void *)((int)this + 0x6c),(ushort)((short)uVar1 << 6) | 9);
  if (this_00 != (void *)0x0) {
    iVar7 = 0;
    uVar6 = param_2;
    iVar5 = FUN_004583e0(this_00,(int *)puVar2);
    FUN_00458040(this_00,iVar5,piVar3,uVar6,iVar7);
  }
  return;
}

