
void * __thiscall FUN_005fbda0(void *this,int param_1,int param_2,uint *param_3)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  undefined4 *puVar4;
  int iVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar1 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655ecb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0060e5e0((void *)((int)this + 9),0,0,0);
  local_4 = 0;
  uVar2 = FUN_005fc100(param_3);
  uVar3 = FUN_005fc100(param_3);
  if ((int)uVar3 < 9) {
    param_3 = (uint *)(param_3[4] << 2);
  }
  else {
    param_3 = (uint *)0x0;
  }
  uVar2 = ((int)(uVar2 * param_1 + 0x1f) >> 3 & 0xfffffffcU) * param_2 + 0x28 + (int)param_3;
  FUN_005fc500(this,puVar1[6],4);
  *(uint *)((int)this + 0x10) = puVar1[4];
  puVar4 = (undefined4 *)FUN_00618b70(uVar2);
  puVar6 = puVar4;
  for (uVar2 = uVar2 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
    *puVar6 = 0;
    puVar6 = puVar6 + 1;
  }
  for (iVar5 = 0; iVar5 != 0; iVar5 = iVar5 + -1) {
    *(undefined1 *)puVar6 = 0;
    puVar6 = (undefined4 *)((int)puVar6 + 1);
  }
  puVar6 = (undefined4 *)*puVar1;
  puVar7 = puVar4;
  for (uVar2 = (uint)(param_3 + 10) >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
    *puVar7 = *puVar6;
    puVar6 = puVar6 + 1;
    puVar7 = puVar7 + 1;
  }
  for (iVar5 = 0; iVar5 != 0; iVar5 = iVar5 + -1) {
    *(undefined1 *)puVar7 = *(undefined1 *)puVar6;
    puVar6 = (undefined4 *)((int)puVar6 + 1);
    puVar7 = (undefined4 *)((int)puVar7 + 1);
  }
  puVar4[1] = param_1;
  puVar4[2] = param_2;
  puVar4[5] = 0;
  puVar4[4] = 0;
  *(undefined4 **)this = puVar4;
  *(undefined1 **)((int)this + 4) = (undefined1 *)((int)(param_3 + 10) + (int)puVar4);
  FUN_005fd170(this);
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  ExceptionList = local_c;
  return this;
}

