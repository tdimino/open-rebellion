
void __thiscall FUN_0051fcb0(void *this,undefined4 *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  void *unaff_ESI;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  puVar1 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006433c8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = *(undefined4 *)((int)this + 0x20);
                    /* WARNING: Load size is inaccurate */
  puVar2 = (undefined4 *)(**(code **)(*this + 0x30))(&param_1);
  puStack_8 = (undefined1 *)0x0;
  FUN_004f26d0(puVar1 + 2,puVar2);
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
                    /* WARNING: Load size is inaccurate */
  uVar3 = (**(code **)(*this + 0xc))();
  puVar1[3] = uVar3;
                    /* WARNING: Load size is inaccurate */
  uVar3 = (**(code **)(*this + 0x40))();
  puVar1[1] = uVar3;
  ExceptionList = unaff_ESI;
  return;
}

