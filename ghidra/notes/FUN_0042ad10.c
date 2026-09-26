
void __thiscall FUN_0042ad10(void *this,uint *param_1,int param_2,int param_3,int param_4)

{
  uint uVar1;
  uint *puVar2;
  int *piVar3;
  uint *puVar4;
  void *this_00;
  
  puVar2 = param_1;
  piVar3 = FUN_004f3220(*(int *)((int)this + 0x9c),param_1);
  puVar4 = FUN_004025b0((void *)piVar3[7],(uint *)&param_1);
  uVar1 = *puVar4;
  FUN_00619730();
  this_00 = (void *)FUN_00604500((void *)((int)this + 0x6c),(ushort)((short)uVar1 << 6) | 1);
  FUN_0042b330(this,puVar2,piVar3,1);
  if (this_00 != (void *)0x0) {
    if (param_2 == 1) {
      FUN_0045c240(this_00,puVar2,param_4,param_3);
    }
    else if (param_2 == 2) {
      FUN_0045baf0(this_00,puVar2,param_4,param_3);
      return;
    }
  }
  return;
}

