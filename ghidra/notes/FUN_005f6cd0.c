
void * __thiscall
FUN_005f6cd0(void *this,char *param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            int param_5,int param_6,undefined4 param_7,undefined4 param_8,undefined4 param_9)

{
  char cVar1;
  HCURSOR pHVar2;
  HGDIOBJ pvVar3;
  int iVar4;
  uint uVar5;
  char *pcVar6;
  char *pcVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  uint uVar11;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00655818;
  pvStack_c = ExceptionList;
  uVar11 = 0;
  uVar10 = 0;
  uVar9 = 0;
  ExceptionList = &pvStack_c;
  pHVar2 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f00);
  uVar8 = 0;
  pvVar3 = GetStockObject(4);
  FUN_005ff910(this,param_2,s_Smacker_Window_006ac268,0x50000000,param_3,param_4,param_5,param_6,
               param_7,(char *)0x0,pvVar3,uVar8,(int)pHVar2,uVar9,uVar10,uVar11);
  uVar11 = 0xffffffff;
  *(undefined ***)this = &PTR_FUN_0066ddb0;
  do {
    pcVar6 = param_1;
    if (uVar11 == 0) break;
    uVar11 = uVar11 - 1;
    pcVar6 = param_1 + 1;
    cVar1 = *param_1;
    param_1 = pcVar6;
  } while (cVar1 != '\0');
  uVar11 = ~uVar11;
  pcVar6 = pcVar6 + -uVar11;
  pcVar7 = (char *)((int)this + 0x94);
  for (uVar5 = uVar11 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
    *(undefined4 *)pcVar7 = *(undefined4 *)pcVar6;
    pcVar6 = pcVar6 + 4;
    pcVar7 = pcVar7 + 4;
  }
  uStack_4 = 0;
  for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
    *pcVar7 = *pcVar6;
    pcVar6 = pcVar6 + 1;
    pcVar7 = pcVar7 + 1;
  }
  *(undefined4 *)((int)this + 0x194) = 0;
  *(undefined4 *)((int)this + 0x198) = 0;
  *(undefined4 *)((int)this + 0x19c) = 1;
  *(undefined4 *)((int)this + 0x1a8) = 0;
  *(undefined4 *)((int)this + 0x1ac) = 0;
  *(undefined4 *)((int)this + 0x1a4) = 0;
  *(undefined4 *)((int)this + 0x1b0) = 0;
  *(undefined4 *)((int)this + 0x1b4) = 0;
  *(undefined4 *)((int)this + 0x1a0) = param_8;
  *(undefined4 *)((int)this + 0x1b8) = param_9;
  *(undefined4 *)((int)this + 0x1c0) = 0;
  *(undefined4 *)((int)this + 0x1bc) = 0;
  iVar4 = FUN_00610c10();
  if (iVar4 == 0) {
    FUN_00610be0(*(undefined4 *)((int)this + 0x18),1,0);
  }
  uVar8 = thunk_FUN_006112f0();
  *(undefined4 *)((int)this + 0x1bc) = uVar8;
  FUN_00610c00(0);
  ExceptionList = pvStack_c;
  return this;
}

