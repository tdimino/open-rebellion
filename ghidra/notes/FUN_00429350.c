
void __thiscall FUN_00429350(void *this,int *param_1)

{
  int *this_00;
  int iVar1;
  void *pvVar2;
  uint *puVar3;
  uint uStack_18;
  undefined4 auStack_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062c380;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  this_00 = (int *)FUN_005f5060((int)this + 0x6c);
  iVar1 = (**(code **)(*param_1 + 0x28))();
  FUN_004f4390(auStack_14,iVar1);
  puVar3 = &uStack_18;
  iStack_4 = 0;
  pvVar2 = (void *)thunk_FUN_005f5060((int)auStack_14);
  FUN_00403040(pvVar2,puVar3);
  iStack_4._0_1_ = 1;
  do {
    if (this_00 == (int *)0x0) {
LAB_0042940a:
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      FUN_00619730();
      iStack_4 = 0xffffffff;
      FUN_004f4380(auStack_14);
      ExceptionList = pvStack_c;
      return;
    }
    pvVar2 = FUN_004f5940(this_00 + 0x45,&uStack_18);
    if ((pvVar2 != (void *)0x0) && (((byte)this_00[9] & 0x3f) == 9)) {
      FUN_00458b50(this_00,param_1);
      goto LAB_0042940a;
    }
    pvVar2 = FUN_004f5940(this_00 + 0x45,&uStack_18);
    if ((pvVar2 != (void *)0x0) && (this_00[6] == *(int *)((int)this + 0xb8))) {
      (**(code **)(*this_00 + 0x78))(param_1);
      goto LAB_0042940a;
    }
    this_00 = (int *)this_00[4];
  } while( true );
}

