
undefined4 __fastcall FUN_0058b060(int param_1)

{
  void *this;
  void *this_00;
  short *psVar1;
  short *psVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  int iVar5;
  undefined4 *puVar6;
  undefined4 local_648;
  undefined4 local_644;
  undefined4 local_640 [400];
  
  for (this = (void *)FUN_00505ef0(); this != (void *)0x0; this = (void *)FUN_0051a5d0(this)) {
    puVar4 = local_640;
    for (iVar5 = 400; iVar5 != 0; iVar5 = iVar5 + -1) {
      *puVar4 = 0;
      puVar4 = puVar4 + 1;
    }
    for (this_00 = (void *)FUN_00505ef0(); this_00 != (void *)0x0;
        this_00 = (void *)FUN_0051a5d0(this_00)) {
      iVar5 = *(int *)((int)this_00 + 0x18) + -100;
      local_640[iVar5 * 2] = this_00;
      psVar1 = (short *)FUN_00509620(this_00,&local_644);
      psVar2 = (short *)FUN_00509620(this,&local_648);
      uVar3 = FUN_0055d860(psVar2,psVar1);
      local_640[iVar5 * 2 + 1] = uVar3;
    }
    FUN_0061a0c0((undefined1 *)local_640,200,8,&LAB_0058b140);
    puVar6 = local_640;
    iVar5 = 200;
    puVar4 = (undefined4 *)(param_1 + 4 + (*(int *)((int)this + 0x18) + -100) * 0x324);
    do {
      uVar3 = *puVar6;
      puVar6 = puVar6 + 2;
      *puVar4 = uVar3;
      puVar4 = puVar4 + 1;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
  }
  return 1;
}

