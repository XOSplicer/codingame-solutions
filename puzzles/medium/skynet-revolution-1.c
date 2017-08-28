#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

 //Node structure
 struct node {
    int id;
    int isGateway;
 };

 //Link structure
 struct link {
    struct node *n1, *n2;
    int isCut;
    int isGatewayLink;
 };

 int cutLink(struct link *l) {
    if(!(l->isCut)) {
        fprintf(stderr, "cutting link [%d]-[%d] (%d)\n", l->n1->id, l->n2->id, l->isGatewayLink);
        printf("%d %d\n", l->n1->id, l->n2->id);
        return 0;
    }
    else {
        fprintf(stderr, "link [%d]-[%d] (%d) is already cut\n", l->n1->id, l->n2->id, l->isGatewayLink);
        return 1;
    }
 }

//Main function
int main()
{
    //Load initial info
    int N; // the total number of nodes in the level, including the gateways
    int L; // the number of links
    int E; // the number of exit gateways
    scanf("%d%d%d", &N, &L, &E);
    fprintf(stderr, "loaded initial info\n");

    //Load nodes
    struct node nodes[N];
    for(int i=0; i<N; i++) {
        nodes[i].id=i;
        nodes[i].isGateway=0;
    }
    fprintf(stderr, "loaded nodes\n");

    //Load links
    struct link links[L];
    for (int i = 0; i < L; i++) {
        int N1; // N1 and N2 defines a link between these nodes
        int N2;
        scanf("%d%d", &N1, &N2);
        links[i].n1=&nodes[N1];
        links[i].n2=&nodes[N2];
        links[i].isCut=0;
        links[i].isGatewayLink=0;
    }
    fprintf(stderr, "loaded links\n");

    //Load gateways
    for (int i = 0; i < E; i++) {
        int EI; // the index of a gateway node
        scanf("%d", &EI);
        nodes[EI].isGateway=1;
    }
    fprintf(stderr, "loaded gateways\n");

    //Mark gateway links
    for(int i=0; i<L; i++) {
        if((links[i].n1->isGateway)||(links[i].n2->isGateway)) {
            links[i].isGatewayLink=1;
        }
    }
    fprintf(stderr, "marked gateway links\n");
    fprintf(stderr, "links are [n1]-[n2] (isGatewayLink) (is Cut)\n");
    for(int i=0; i<L; i++) {
        fprintf(stderr, "[%d]-[%d] (%d) (%d)\n", links[i].n1->id, links[i].n2->id, links[i].isGatewayLink, links[i].isCut);
    }

    fprintf(stderr, "done setup\n\n");

    // game loop
    while (1) {
        int SI; // The index of the node on which the Skynet agent is positioned this turn
        scanf("%d", &SI);
        int cut=0;

        //cut direct access to gateway
        //avoid double cut
        for(int i=0; i<L; i++) {
            if((!links[i].isCut)&&((links[i].n1->id==SI)||(links[i].n2->id==SI))&&(!cut)&&(links[i].isGatewayLink)) {
                fprintf(stderr, "cutting gateway link\n");
                cutLink(&links[i]);
                cut=1;
            }
        }

        //cut next to agent if no gateway present
        //avoid double cut
        if(!cut) {
            for(int i=0; i<L; i++) {
                if((!links[i].isCut)&&((links[i].n1->id==SI)||(links[i].n2->id==SI))&&(!cut)) {
                fprintf(stderr, "cutting next link\n");
                cutLink(&links[i]);
                cut=1;
            }
        }
        }



    }

    return 0;
}