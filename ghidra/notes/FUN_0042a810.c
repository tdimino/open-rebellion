
void __thiscall FUN_0042a810(void *this,uint *param_1)

{
  uint uVar1;
  uint *puVar2;
  int *piVar3;
  uint *puVar4;
  void *this_00;
  undefined4 *puVar5;
  
  puVar2 = param_1;
  piVar3 = FUN_004f2f60(*(int *)((int)this + 0x9c),param_1);
  puVar4 = FUN_004025b0((void *)piVar3[7],(uint *)&param_1);
  uVar1 = *puVar4;
  FUN_00619730();
  this_00 = (void *)FUN_00604500((void *)((int)this + 0x6c),(ushort)((short)uVar1 << 6) | 9);
  if (this_00 != (void *)0x0) {
    puVar5 = (undefined4 *)FUN_004583e0(this_00,(int *)puVar2);
    FUN_00457c90(this_00,puVar5,piVar3);
  }
  FUN_0042abd0(this,puVar2,(int *)0x0);
  return;
}

